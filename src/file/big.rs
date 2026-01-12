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

use crate::{Authenticated, Client};
use reqwest::{
    Response,
    header::AUTHORIZATION,
    multipart::{Form, Part},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::io::AsyncRead;
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;

#[derive(Deserialize)]
struct CreateResponse {
    id: String,
}

impl Client<Authenticated> {
    async fn create_big_file(&self) -> anyhow::Result<String> {
        let res = self
            .http
            .get(format!("{}/files/big/create", self.base_url))
            .header(AUTHORIZATION, &self.state.token)
            .send()
            .await?
            .error_for_status()?;
        let data = res.json::<CreateResponse>().await?;
        Ok(data.id)
    }
}

pub struct BigFile<'a> {
    id: String,
    client: &'a Client<Authenticated>,
}

#[derive(Serialize)]
struct MoveBody<'b> {
    id: &'b str,
    pathname: PathBuf,
}

pub enum AfterUpload {
    Import,
    Move { pathname: PathBuf },
}

impl<'a> BigFile<'a> {
    pub async fn new(client: &'a Client<Authenticated>) -> anyhow::Result<Self> {
        let id = client.create_big_file().await?;
        Ok(Self { client, id })
    }

    async fn append(&self, chunk: Vec<u8>) -> anyhow::Result<Response> {
        let form = Form::new().text("id", self.id.clone()).part(
            "file",
            Part::bytes(chunk).mime_str("application/octet-stream")?,
        );

        let res = self.client.multipart("/files/big/append", form).await?;

        Ok(res)
    }

    pub async fn upload<R>(&self, reader: R, after: AfterUpload) -> anyhow::Result<Response>
    where
        R: AsyncRead + Send + Sync + Unpin + 'static,
    {
        let mut stream = ReaderStream::new(reader);

        while let Some(bytes) = stream.next().await {
            let mut chunk = Vec::new();
            chunk.extend_from_slice(&bytes?);
            self.append(chunk).await?;
        }

        match after {
            AfterUpload::Move { pathname } => {
                let body = MoveBody {
                    pathname,
                    id: &self.id,
                };

                let res = self.client.post("/files/big/move", &body).await?;

                Ok(res)
            }
            AfterUpload::Import => {
                let res = self
                    .client
                    .http
                    .post(format!("{}/files/import/{}", self.client.base_url, self.id))
                    .send()
                    .await?
                    .error_for_status()?;

                Ok(res)
            }
        }
    }
}
