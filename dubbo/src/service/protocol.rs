/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::invocation;
use crate::common::url::Url;

use async_trait::async_trait;

#[async_trait]
pub trait Protocol {
    type Invoker;
    type Exporter;

    fn destroy(&self);
    async fn export(self, url: Url) -> Self::Exporter;
    async fn refer(&self, url: Url) -> Self::Invoker;
}

pub trait Exporter {
    type InvokerType: Invoker;

    fn unexport(&self);
    fn get_invoker(&self) -> Self::InvokerType;
}

pub trait Invoker {
    fn invoke<M1>(&self, req: invocation::Request<M1>) -> invocation::Response<String>
    where
        M1: Send + 'static;
    fn is_available(&self) -> bool;
    fn destroy(&self);
    fn get_url(&self) -> Url;
}
