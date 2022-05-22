use capnp::capability::Promise;
use capnp_rpc::pry;
use log::trace;

pub mod diamond_capnp {
    include!(concat!(env!("OUT_DIR"), "/diamond_capnp.rs"));
}

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
        Self { limit, c: 1 }
    }
}
impl diamond_capnp::counter::Server for CounterImpl {
    fn next(
        &mut self,
        _: diamond_capnp::counter::NextParams,
        mut results: diamond_capnp::counter::NextResults,
    ) -> Promise<(), capnp::Error> {
        trace!("next: {}", self.c > self.limit);
        results.get().set_exist(self.c > self.limit);
        self.c += 1;

        Promise::ok(())
    }
    fn get_count(
        &mut self,
        _: diamond_capnp::counter::GetCountParams,
        mut results: diamond_capnp::counter::GetCountResults,
    ) -> Promise<(), capnp::Error> {
        trace!("get_count c: {}", self.c);
        results.get().set_count(self.c);

        Promise::ok(())
    }
}
