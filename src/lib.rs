use std::{thread, time::Duration};

use capnp::capability::Promise;
use capnp_rpc::pry;
use log::trace;

pub mod diamond_capnp {
    include!(concat!(env!("OUT_DIR"), "/diamond_capnp.rs"));
}

pub const SHORT_SLEEP_SECS: u64 = 1;

pub struct FooImpl;
impl FooImpl {
    pub fn new() -> Self {
        Self
    }
}
impl diamond_capnp::foo::Server for FooImpl {
    fn get_bar(
        &mut self,
        params: diamond_capnp::foo::GetBarParams,
        mut results: diamond_capnp::foo::GetBarResults,
    ) -> Promise<(), capnp::Error> {
        let name = pry!(pry!(params.get()).get_name());
        trace!("get_bar name: {}", name);
        let bar: diamond_capnp::bar::Client = capnp_rpc::new_client(BarImpl::new(name));
        results.get().set_bar(bar);

        trace!("get_bar name called");
        Promise::ok(())
    }

    fn get_baz(
        &mut self,
        params: diamond_capnp::foo::GetBazParams,
        mut results: diamond_capnp::foo::GetBazResults,
    ) -> Promise<(), capnp::Error> {
        let age = pry!(params.get()).get_age();
        trace!("get_baz age: {}", age);
        let baz: diamond_capnp::baz::Client = capnp_rpc::new_client(BazImpl::new(age));
        results.get().set_baz(baz);

        trace!("get_baz age called");
        Promise::ok(())
    }
    fn get_counter(
        &mut self,
        params: diamond_capnp::foo::GetCounterParams,
        mut results: diamond_capnp::foo::GetCounterResults,
    ) -> Promise<(), capnp::Error> {
        let limit = pry!(params.get()).get_limit();
        trace!("get_counter limit: {}", limit);
        let counter: diamond_capnp::counter::Client =
            capnp_rpc::new_client(CounterImpl::new(limit));
        results.get().set_counter(counter);

        trace!("get_counter called");
        Promise::ok(())
    }
    fn get_naive_counter(
        &mut self,
        params: diamond_capnp::foo::GetNaiveCounterParams,
        mut results: diamond_capnp::foo::GetNaiveCounterResults,
    ) -> Promise<(), capnp::Error> {
        let limit = pry!(params.get()).get_limit();
        trace!("get_naive_counter limit: {}", limit);
        let counter: diamond_capnp::naive_counter::Client =
            capnp_rpc::new_client(NaiveCounterImpl::new(limit));
        results.get().set_naive_counter(counter);

        trace!("get_naive_counter called");
        Promise::ok(())
    }
}

pub struct BarImpl {
    name: String,
}
impl BarImpl {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
impl diamond_capnp::bar::Server for BarImpl {
    fn read_val(
        &mut self,
        _: diamond_capnp::bar::ReadValParams,
        mut results: diamond_capnp::bar::ReadValResults,
    ) -> Promise<(), capnp::Error> {
        trace!("get_bar read_val");
        results.get().set_val(self.name.as_str().into());

        trace!("get_bar read_val called");
        Promise::ok(())
    }
}

pub struct BazImpl {
    age: u16,
}
impl BazImpl {
    pub fn new(age: u16) -> Self {
        Self { age }
    }
}
impl diamond_capnp::baz::Server for BazImpl {
    fn read_val(
        &mut self,
        _: diamond_capnp::baz::ReadValParams,
        mut results: diamond_capnp::baz::ReadValResults,
    ) -> Promise<(), capnp::Error> {
        trace!("get_baz read_val");
        results.get().set_val(self.age);

        trace!("get_baz read_val called");
        Promise::ok(())
    }
}

pub struct QuxImpl;
impl QuxImpl {
    pub fn new() -> Self {
        Self
    }
}
impl diamond_capnp::qux::Server for QuxImpl {
    fn calc(
        &mut self,
        params: diamond_capnp::qux::CalcParams,
        mut results: diamond_capnp::qux::CalcResults,
    ) -> Promise<(), capnp::Error> {
        trace!("get_qux calc");
        let bar = pry!(pry!(params.get()).get_bar());
        let name: Promise<String, capnp::Error> = Promise::from_future(async move {
            Ok(bar
                .read_val_request()
                .send()
                .promise
                .await?
                .get()?
                .get_val()?
                .to_string())
        });

        let baz = pry!(pry!(params.get()).get_baz());
        let age: Promise<u16, capnp::Error> = Promise::from_future(async move {
            Ok(baz
                .read_val_request()
                .send()
                .promise
                .await?
                .get()?
                .get_val())
        });

        Promise::from_future(async move {
            let age = age.await?;
            let desc = if age >= 18 { "Adult" } else { "Child" };
            results.get().set_age(age);
            results
                .get()
                .set_name(&format!("{}({})", name.await?.as_str(), desc));

            trace!("calc called");
            Ok(())
        })
    }
}

pub struct CounterImpl {
    limit: u16,
    c: u16,
}
impl CounterImpl {
    pub fn new(limit: u16) -> Self {
        Self { limit, c: 0 }
    }
}
impl diamond_capnp::counter::Server for CounterImpl {
    fn next(
        &mut self,
        _: diamond_capnp::counter::NextParams,
        mut results: diamond_capnp::counter::NextResults,
    ) -> Promise<(), capnp::Error> {
        self.c += 1;
        let b = self.c <= self.limit;
        trace!("next: {}, c: {}", b, self.c);
        let boolbox_client: diamond_capnp::bool_box::Client =
            capnp_rpc::new_client(BoolBoxImpl::new(b));
        results.get().set_exist(boolbox_client);

        trace!("next called");
        Promise::ok(())
    }
    fn get_count(
        &mut self,
        _: diamond_capnp::counter::GetCountParams,
        mut results: diamond_capnp::counter::GetCountResults,
    ) -> Promise<(), capnp::Error> {
        trace!("get_count c: {}", self.c);
        results.get().set_count(self.c);

        trace!("get_count called");
        Promise::ok(())
    }
    fn run_fast(
        &mut self,
        _: diamond_capnp::counter::RunFastParams,
        mut results: diamond_capnp::counter::RunFastResults,
    ) -> Promise<(), capnp::Error> {
        trace!("run_fast");
        Promise::from_future(async move {
            let counter_client: diamond_capnp::counter::Client =
                capnp_rpc::new_client(CounterImpl::new(20));
            while counter_client
                .next_request()
                .send()
                .pipeline
                .get_exist()
                .get_raw_request()
                .send()
                .promise
                .await?
                .get()?
                .get_raw()
            {
                thread::sleep(Duration::from_secs(SHORT_SLEEP_SECS));
                let c = counter_client
                    .get_count_request()
                    .send()
                    .promise
                    .await?
                    .get()?
                    .get_count();
                results.get().set_count(c);
                trace!("---");
            }

            trace!("run_fast called");
            Ok(())
        })
    }
}

pub struct BoolBoxImpl {
    b: bool,
}
impl BoolBoxImpl {
    pub fn new(b: bool) -> Self {
        Self { b }
    }
}
impl diamond_capnp::bool_box::Server for BoolBoxImpl {
    fn get_raw(
        &mut self,
        _: diamond_capnp::bool_box::GetRawParams,
        mut results: diamond_capnp::bool_box::GetRawResults,
    ) -> Promise<(), capnp::Error> {
        trace!("get_raw: {}", self.b);
        results.get().set_raw(self.b);

        trace!("get_raw called");
        Promise::ok(())
    }
}

pub struct NaiveCounterImpl {
    limit: u16,
    c: u16,
}
impl NaiveCounterImpl {
    pub fn new(limit: u16) -> Self {
        Self { limit, c: 0 }
    }
}
impl diamond_capnp::naive_counter::Server for NaiveCounterImpl {
    fn next(
        &mut self,
        _: diamond_capnp::naive_counter::NextParams,
        mut results: diamond_capnp::naive_counter::NextResults,
    ) -> Promise<(), capnp::Error> {
        self.c += 1;
        let b = self.c <= self.limit;
        trace!("next: {}, c: {}", b, self.c);
        results.get().set_exist(b);

        trace!("next called");
        Promise::ok(())
    }
    fn get_count(
        &mut self,
        _: diamond_capnp::naive_counter::GetCountParams,
        mut results: diamond_capnp::naive_counter::GetCountResults,
    ) -> Promise<(), capnp::Error> {
        trace!("get_count c: {}", self.c);
        results.get().set_count(self.c);

        trace!("get_count called");
        Promise::ok(())
    }
}
