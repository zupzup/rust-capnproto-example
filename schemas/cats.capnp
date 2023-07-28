@0x9d62d5f53cf92b66;


struct Cat {
    name @0: Text;
    age @1: UInt8;
    color @2: Text;
    cuteness @3: Float32;
    addresses@4: List(Address);

    struct Address {
        street @0: Text;
        number @1: UInt8;
        postalcode @2: UInt16;
    }

    image @5: Data;
}
