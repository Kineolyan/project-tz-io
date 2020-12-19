use nom::bytes::complete::tag;
use nom::IResult; //space;

use crate::address::port_ref;
use crate::common::{be_uint, ws};
use language::syntax::{InputMapping, OutputMapping};

pub fn input_item(input: &[u8]) -> IResult<&[u8], InputMapping> {
    let (remaining, (port, _, input_ref)) =
        nom::sequence::tuple((port_ref, ws(tag("->")), be_uint))(input)?;
    let mapping = InputMapping {
        from: port,
        to: input_ref,
    };
    Ok((remaining, mapping))
}

pub fn inputs(input: &[u8]) -> IResult<&[u8], Vec<InputMapping>> {
    nom::multi::separated_list1(ws(tag(",")), input_item)(input)
}

pub fn output_item(input: &[u8]) -> IResult<&[u8], OutputMapping> {
    let (remaining, (input_ref, _, port)) =
        nom::sequence::tuple((be_uint, ws(tag("->")), port_ref))(input)?;
    let mapping = OutputMapping {
        from: input_ref,
        to: port,
    };
    Ok((remaining, mapping))
}

pub fn outputs(input: &[u8]) -> IResult<&[u8], Vec<OutputMapping>> {
    nom::multi::separated_list1(ws(tag(",")), output_item)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::tests::*;
    use crate::common::to_input;
    use language::address::{Node, Port};

    #[test]
    fn test_parse_input_item() {
        let res_in = input_item(to_input(b"IN:1 -> 3"));
        assert_full_result(
            res_in,
            InputMapping {
                from: Port::new(Node::In, 1),
                to: 3u32,
            },
        );

        let res_node = input_item(to_input(b"#node:32 -> 1"));
        assert_full_result(
            res_node,
            InputMapping {
                from: Port::named_port(&"node", 32),
                to: 1u32,
            },
        );
    }

    #[test]
    fn test_parse_inputs() {
        let res_one = inputs(to_input(b"#n:7 -> 14"));
        assert_full_result(
            res_one,
            vec![InputMapping {
                from: Port::named_port(&"n", 7),
                to: 14u32,
            }],
        );

        let res_many = inputs(to_input(b"OUT:1 -> 2, #abc:3 -> 4"));
        assert_full_result(
            res_many,
            vec![
                InputMapping {
                    from: Port::new(Node::Out, 1),
                    to: 2u32,
                },
                InputMapping {
                    from: Port::named_port(&"abc", 3),
                    to: 4u32,
                },
            ],
        );
    }

    #[test]
    fn test_parse_output_item() {
        let res_in = output_item(to_input(b"1 -> OUT:3"));
        assert_full_result(
            res_in,
            OutputMapping {
                from: 1u32,
                to: Port::new(Node::Out, 3),
            },
        );

        let res_node = output_item(to_input(b"1 -> #node:32"));
        assert_full_result(
            res_node,
            OutputMapping {
                from: 1u32,
                to: Port::named_port(&"node", 32),
            },
        );
    }

    #[test]
    fn test_parse_outputs() {
        let res_one = outputs(to_input(b"3 -> #n:7"));
        assert_full_result(
            res_one,
            vec![OutputMapping {
                from: 3,
                to: Port::named_port(&"n", 7),
            }],
        );

        let res_many = outputs(to_input(b"1 -> OUT:2, 3 -> #abc:4"));
        assert_full_result(
            res_many,
            vec![
                OutputMapping {
                    from: 1u32,
                    to: Port::new(Node::Out, 2),
                },
                OutputMapping {
                    from: 3u32,
                    to: Port::named_port(&"abc", 4),
                },
            ],
        );
    }
}
