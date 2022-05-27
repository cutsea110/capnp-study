@0xd35bf1654de4b048;

interface Foo {
  getBar          @0 (name :Text) -> (bar :Bar);
  getBaz          @1 (age :UInt16) -> (baz :Baz);
  getCounter      @2 (limit :UInt16) -> (counter :Counter);
  getNaiveCounter @3 (limit :UInt16) -> (naiveCounter :NaiveCounter);
  getRose         @4 (depth :UInt16) -> (rose :Rose);
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

interface BoolBox {
  getRaw @0 () -> (raw :Bool);
}

interface Counter {
  next     @0 () -> (exist :BoolBox);
  getCount @1 (ok :BoolBox) -> (count :UInt16);
  runFast  @2 () -> (count :UInt16);
}

interface NaiveCounter {
  next     @0 () -> (exist :Bool);
  getCount @1 () -> (count :UInt16);
}

interface Rose {
  shape   @0 () -> (s :Shape);
  color   @1 () -> (color :Color);
  getName @2 () -> (name :Text);
  getAge  @3 () -> (age :UInt16);
  getSub  @4 () -> (sub :List(Rose));

  enum Color {
    red   @0;
    green @1;
    blue  @2;
  }
  struct Shape {
    union {
      circle    @0 :Circle;
      rectangle @1 :Rectangle;
    }
  }
  interface Circle {
    getRadius @0 () -> (r :UInt16);
  }
  interface Rectangle {
    getWidth  @0 () -> (w :UInt16);
    getHeight @1 () -> (h :UInt16);
  }
}
