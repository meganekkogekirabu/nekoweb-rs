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

use crate::{Authenticated, Client, Unauthenticated};
use reqwest::Response;
use serde::Deserialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Clone)]
struct SiteResponse {
    domain: String,
    updates: u64,
    followers: u64,
    views: u64,
    created_at: u64,
    updated_at: u64,
}

#[derive(Debug, Clone)]
pub struct Site {
    pub domain: String,
    pub updates: u64,
    pub followers: u64,
    pub views: u64,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

async fn get_site(res: Response) -> anyhow::Result<Site> {
    let data = res.json::<SiteResponse>().await?;

    Ok(Site {
        domain: data.domain,
        updates: data.updates,
        followers: data.followers,
        views: data.views,
        created_at: UNIX_EPOCH + Duration::from_millis(data.created_at),
        updated_at: UNIX_EPOCH + Duration::from_millis(data.updated_at),
    })
}

impl Client<Authenticated> {
    pub async fn get_site(&self, username: Option<&str>) -> anyhow::Result<Site> {
        let path = username.map(|s| format!("/{s}")).unwrap_or_default();
        let res = self.get(format!("/site/info{path}")).await?;
        get_site(res).await
    }
}

impl Client<Unauthenticated> {
    pub async fn get_site(&self, username: &str) -> anyhow::Result<Site> {
        let res = self.get(format!("/site/info/{username}")).await?;
        get_site(res).await
    }
}
