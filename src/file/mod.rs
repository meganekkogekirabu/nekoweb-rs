/*
 * Copyright (c) 2026 Choi Madeleine
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#![allow(non_snake_case)]

use crate::{Authenticated, Client};
use reqwest::{
    Response,
    multipart::{Form, Part},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Component, PathBuf},
};
use tokio::io::AsyncRead;

mod big;
use big::{AfterUpload, BigFile};

#[derive(Serialize)]
struct CreateBody {
    isFolder: bool,
    pathname: String,
}

#[derive(Serialize)]
struct RenameBody {
    pathname: String,
    newpathname: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct File {
    pub name: PathBuf,
    pub dir: bool,
}

impl Client<Authenticated> {
    async fn create(&self, pathname: &str, isFolder: bool) -> anyhow::Result<Response> {
        let pathname = pathname.into();
        let body = CreateBody { pathname, isFolder };
        Ok(self.post("/files/create", &body).await?)
    }

    pub async fn create_file(&self, path: &str) -> anyhow::Result<Response> {
        Ok(self.create(path, false).await?)
    }

    pub async fn create_folder(&self, path: &str) -> anyhow::Result<Response> {
        Ok(self.create(path, true).await?)
    }

    pub async fn upload_file(
        &self,
        path: impl Into<PathBuf>,
        buffer: Vec<u8>,
    ) -> anyhow::Result<Response> {
        let pathname = path.into();

        let mut parts = pathname
            .components()
            .filter_map(|c| match c {
                Component::Normal(os) => os.to_str().map(String::from),
                _ => None,
            })
            .collect::<Vec<String>>();

        let filename = parts.pop().unwrap_or("file.bin".to_string());

        let mut dirname = PathBuf::from("/");
        for part in parts {
            dirname.push(part);
        }

        let form = Form::new()
            .text("pathname", dirname.to_string_lossy().into_owned())
            .part(
                "files",
                Part::bytes(buffer)
                    .file_name(filename)
                    .mime_str("application/octet_stream")?,
            );

        let res = self.multipart("/files/upload", form).await?;

        Ok(res)
    }

    pub async fn upload_stream<R>(
        &self,
        pathname: impl Into<PathBuf>,
        reader: R,
    ) -> anyhow::Result<Response>
    where
        R: AsyncRead + Send + Sync + Unpin + 'static,
    {
        let bigfile = BigFile::new(self).await?;
        bigfile
            .upload(
                reader,
                AfterUpload::Move {
                    pathname: pathname.into(),
                },
            )
            .await
    }

    pub async fn import_stream<R>(&self, reader: R) -> anyhow::Result<Response>
    where
        R: AsyncRead + Send + Sync + Unpin + 'static,
    {
        let bigfile = BigFile::new(self).await?;
        bigfile.upload(reader, AfterUpload::Import).await
    }

    pub async fn list(&self, pathname: &str) -> anyhow::Result<Vec<File>> {
        let res = self
            .get(format!(
                "/files/readfolder?{}",
                serde_urlencoded::to_string(&HashMap::from([("pathname", &pathname)]))?
            ))
            .await?;

        let files = res.json::<Vec<File>>().await?;

        Ok(files)
    }

    pub async fn rename(&self, from: &str, to: &str) -> anyhow::Result<Response> {
        let body = RenameBody {
            pathname: from.into(),
            newpathname: to.into(),
        };

        let res = self.post("/files/rename", &body).await?;

        Ok(res)
    }

    pub async fn edit(&self, pathname: String, content: Vec<u8>) -> anyhow::Result<Response> {
        let form = Form::new()
            .text("pathname", pathname)
            .part("content", Part::bytes(content));

        let res = self.multipart("/files/edit", form).await?;

        Ok(res)
    }

    pub async fn delete(&self, pathname: String) -> anyhow::Result<Response> {
        let res = self
            .post("/files/delete", &HashMap::from([("pathname", &pathname)]))
            .await?;
        Ok(res)
    }
}
