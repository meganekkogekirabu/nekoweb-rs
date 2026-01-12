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

use reqwest::{Response, header::AUTHORIZATION, multipart::Form};
use serde::Serialize;

pub mod file;
pub mod site;

pub struct Unauthenticated;
pub struct Authenticated {
    token: String,
}

#[derive(Debug, Clone)]
pub struct Client<S> {
    base_url: String,
    http: reqwest::Client,
    state: S,
}

impl Client<Unauthenticated> {
    pub fn new(user_agent: &str) -> anyhow::Result<Self> {
        Ok(Self {
            base_url: "https://nekoweb.org/api".into(),
            http: reqwest::Client::builder().user_agent(user_agent).build()?,
            state: Unauthenticated,
        })
    }

    pub fn authenticate(self, token: String) -> Client<Authenticated> {
        Client {
            state: Authenticated { token },
            base_url: self.base_url,
            http: self.http,
        }
    }

    async fn get(&self, path: impl Into<String>) -> anyhow::Result<Response> {
        Ok(self
            .http
            .get(format!("{}{}", &self.base_url, path.into()))
            .send()
            .await?
            .error_for_status()?)
    }
}

impl Client<Authenticated> {
    async fn post<T>(&self, path: impl Into<String>, body: &T) -> anyhow::Result<Response>
    where
        T: Serialize,
    {
        Ok(self
            .http
            .post(format!("{}{}", &self.base_url, path.into()))
            .header(AUTHORIZATION, &self.state.token)
            .form(body)
            .send()
            .await?
            .error_for_status()?)
    }

    async fn multipart(&self, path: impl Into<String>, form: Form) -> anyhow::Result<Response> {
        Ok(self
            .http
            .post(format!("{}{}", &self.base_url, path.into()))
            .header(AUTHORIZATION, &self.state.token)
            .multipart(form)
            .send()
            .await?
            .error_for_status()?)
    }

    async fn get_auth(&self, path: impl Into<String>) -> anyhow::Result<Response> {
        Ok(self
            .http
            .get(format!("{}{}", &self.base_url, path.into()))
            .header(AUTHORIZATION, &self.state.token)
            .send()
            .await?
            .error_for_status()?)
    }

    async fn get(&self, path: impl Into<String>) -> anyhow::Result<Response> {
        Ok(self
            .http
            .get(format!("{}{}", &self.base_url, path.into()))
            .send()
            .await?
            .error_for_status()?)
    }
}
