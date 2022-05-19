@0xd35bf1654de4b048;

interface Foo {
  getBar @0 (name :Text) -> (bar :Bar);
  getBuz @1 (age :UInt16) -> (buz :Buz);
}

interface Bar {
  readVal @0 () -> (val :Text);
}

interface Buz {
  readVal @0 () -> (val :UInt16);
}

interface Moo {
  calc @0 (bar :Bar, buz :Buz) -> (name :Text, age :UInt16);
}
