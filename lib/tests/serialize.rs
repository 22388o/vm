#[cfg(test)]
mod serialize_test {
    use bitcoin_vm::serialize::{to_csv_script_encode, to_n_sequence_encode, with_prefix_pushdata, CSVFlag};

    #[test]
    fn test_prefix_pushdata() {
        let data_1 = hex::decode("aa").unwrap();
        let expected_1 = hex::decode("01aa").unwrap();

        assert_eq!(with_prefix_pushdata(&data_1), expected_1);

        let data_2 = hex::decode("aaaa").unwrap();
        let expected_2 = hex::decode("02aaaa").unwrap();

        assert_eq!(with_prefix_pushdata(&data_2), expected_2);

        let data_3 = hex::decode("aaaaaaaaaa").unwrap();
        let expected_3 = hex::decode("05aaaaaaaaaa").unwrap();

        assert_eq!(with_prefix_pushdata(&data_3), expected_3);

        let data_4 = hex::decode(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
        .unwrap();
        let expected_4 = hex::decode("2aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(with_prefix_pushdata(&data_4), expected_4);

        let data_5 = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let expected_5 = hex::decode("4baaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(with_prefix_pushdata(&data_5), expected_5);

        let data_6 = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let expected_6 = hex::decode("4c4daaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(with_prefix_pushdata(&data_6), expected_6);

        let data_7 = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let expected_7 = hex::decode("4d0a01aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(with_prefix_pushdata(&data_7), expected_7);

        // Minimal pushes

        let data_8 = hex::decode("").unwrap();
        let expected_8 = hex::decode("00").unwrap();

        assert_eq!(with_prefix_pushdata(&data_8), expected_8);

        let data_9 = hex::decode("01").unwrap();
        let expected_9 = hex::decode("51").unwrap();

        assert_eq!(with_prefix_pushdata(&data_9), expected_9);

        let data_10 = hex::decode("09").unwrap();
        let expected_10 = hex::decode("59").unwrap();

        assert_eq!(with_prefix_pushdata(&data_10), expected_10);

        let data_11 = hex::decode("0a").unwrap();
        let expected_11 = hex::decode("5a").unwrap();

        assert_eq!(with_prefix_pushdata(&data_11), expected_11);

        let data_12 = hex::decode("0f").unwrap();
        let expected_12 = hex::decode("5f").unwrap();

        assert_eq!(with_prefix_pushdata(&data_12), expected_12);

        let data_13 = hex::decode("10").unwrap();
        let expected_13 = hex::decode("60").unwrap();

        assert_eq!(with_prefix_pushdata(&data_13), expected_13);

        let data_14 = hex::decode("11").unwrap();
        let not_expected_14 = hex::decode("61").unwrap();
        let expected_14 = hex::decode("0111").unwrap();

        assert_eq!(with_prefix_pushdata(&data_14), expected_14);
        assert_ne!(with_prefix_pushdata(&data_14), not_expected_14);
    }


    #[test]
    fn test_csv_days() {
        // 1
        let days = 1;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("90000000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("029000b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 2
        let days = 2;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("20010000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("022001b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 3
        let days = 3;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("b0010000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("02b001b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 4
        let days = 4;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("40020000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("024002b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 5
        let days = 5;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("d0020000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("02d002b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 10
        let days = 10;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("a0050000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("02a005b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 55
        let days = 55;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("f01e0000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("02f01eb275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 100
        let days = 100;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("40380000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("024038b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 200
        let days = 200;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("80700000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("028070b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 220
        let days = 220;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("c07b0000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("02c07bb275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 230
        let days = 230;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("60810000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("03608100b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 240
        let days = 240;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("00870000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("03008700b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 254
        let days = 254;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("e08e0000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("03e08e00b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 254
        let days = 255;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("708f0000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("03708f00b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);
    }
}