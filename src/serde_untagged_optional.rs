use serde::{Deserialize, Deserializer, Serialize, Serializer};
use either::*;


#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

pub fn serialize<L, R, S>(this: &Either<L, R>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    L: Serialize,
    R: Serialize,
{
    let untagged = match this {
        &Either::Left(ref left) => Either::Left(left),
        &Either::Right(ref right) => Either::Right(right),
    };
    untagged.serialize(serializer)
}

pub fn deserialize<'de, L, R, D>(deserializer: D) -> Result<Either<L, R>, D::Error>
where
    D: Deserializer<'de>,
    L: Deserialize<'de>,
    R: Deserialize<'de>,
{
    match Either::deserialize(deserializer) {
        Ok(Either::Left(left)) => Ok(Either::Left(left)),
        Ok(Either::Right(right)) => Ok(Either::Right(right)),
        Err(error) => Err(error),
    }
}