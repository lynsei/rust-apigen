#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum TestEnum {
    True,
    Number_1_5,
    Number_23,
    Number_964,
    Number__79_23,
    Number_14343,
    Number__964,
    Hello,
    Foo,
    Bar,
}
impl Default for TestEnum {
    fn default() -> Self {
        TestEnum::True
    }
}
impl serde::Serialize for TestEnum {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        match self {
            TestEnum::True => (true).serialize(ser),
            TestEnum::Number_1_5 => (1.5).serialize(ser),
            TestEnum::Number_23 => (23).serialize(ser),
            TestEnum::Number_964 => (964).serialize(ser),
            TestEnum::Number__79_23 => (-79.23).serialize(ser),
            TestEnum::Number_14343 => (14343).serialize(ser),
            TestEnum::Number__964 => (-964).serialize(ser),
            TestEnum::Hello => ("hello").serialize(ser),
            TestEnum::Foo => ("foo").serialize(ser),
            TestEnum::Bar => ("bar").serialize(ser),
        }
    }
}
impl<'de> serde::Deserialize<'de> for TestEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deser: D) -> Result<Self, D::Error> {
        use serde::de::{Error, Unexpected, Visitor};
        struct VariantVisitor;
        const EXPECT_MSG: &str = "valid value for enum TestEnum";

        impl<'de> Visitor<'de> for VariantVisitor {
            type Value = TestEnum;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(EXPECT_MSG)
            }

            fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
                if v == true {
                    return Ok(TestEnum::True);
                }
                Err(E::invalid_value(Unexpected::Bool(v), &EXPECT_MSG))
            }

            fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
                if v == -964 {
                    return Ok(TestEnum::Number__964);
                }
                Err(E::invalid_value(Unexpected::Signed(v), &EXPECT_MSG))
            }

            fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
                if v == 23 {
                    return Ok(TestEnum::Number_23);
                }
                if v == 964 {
                    return Ok(TestEnum::Number_964);
                }
                if v == 14343 {
                    return Ok(TestEnum::Number_14343);
                }
                Err(E::invalid_value(Unexpected::Unsigned(v), &EXPECT_MSG))
            }

            fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
                if v == 1.5 {
                    return Ok(TestEnum::Number_1_5);
                }
                if v == -79.23 {
                    return Ok(TestEnum::Number__79_23);
                }
                Err(E::invalid_value(Unexpected::Float(v), &EXPECT_MSG))
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                if v == "hello" {
                    return Ok(TestEnum::Hello);
                }
                if v == "foo" {
                    return Ok(TestEnum::Foo);
                }
                if v == "bar" {
                    return Ok(TestEnum::Bar);
                }
                Err(E::invalid_value(Unexpected::Str(v), &EXPECT_MSG))
            }
        }

        deser.deserialize_any(VariantVisitor)
    }
}

