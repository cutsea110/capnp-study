@0xd35bf1654de4b048;

interface Foo {
  getBar     @0 (name :Text) -> (bar :Bar);
  getBaz     @1 (age :UInt16) -> (baz :Baz);
  getCounter @2 (limit :UInt16) -> (counter :Counter);
}

interface Bar {
  readVal @0 () -> (val :Text);
}

interface Baz {
  readVal @0 () -> (val :UInt16);
}

interface Qux {
  calc @0 (bar :Bar, baz :Baz) -> (name :Text, age :UInt16);
}

interface Counter {
  next     @0 () -> (exist :Bool);
  getCount @1 () -> (count :UInt16);
}
