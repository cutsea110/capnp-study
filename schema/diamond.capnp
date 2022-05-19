@0xd35bf1654de4b048;

interface Foo {
  getBar @0 (name :Text) -> (bar :Bar);
  getBaz @1 (age :UInt16) -> (buz :Baz);
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
