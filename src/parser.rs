use nom::{self, bytes::complete::tag, sequence::tuple, IResult, number, character, multi::many1};

// pub fn parse_verify_image(input: &str) -> IResult<&str, ()> {
//     fn parse_image_verify_length(input: &str) -> IResult<&str, ()> {
//         Ok(tuple((
//             character::complete::u32,
//             tag(" "),
//             many1(number::complete::hex_u32)
//         ))(input))
//     }
//     tuple((
//         tag("Verifying image...\r\n"),
//         tag("Verifying image length="),
//         parse_image_verify_length,
//         tag("\r\n"),
//         tag("SHA256:"),
        
//     ))(input)
// }
