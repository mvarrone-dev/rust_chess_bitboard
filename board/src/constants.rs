use crate::bitboard::{BitBoard};

pub const NUM_BITBOARDS: usize = 22;

pub const WHITE_PAWNS_BB: usize = 0;
pub const WHITE_KNIGHTS_BB: usize = 1;
pub const WHITE_BISHOPS_BB: usize = 2;
pub const WHITE_ROOKS_BB: usize = 3;
pub const WHITE_QUEENS_BB: usize = 4;
pub const WHITE_KINGS_BB: usize = 5;
pub const BLACK_PAWNS_BB: usize = 6;
pub const BLACK_KNIGHTS_BB: usize = 7;
pub const BLACK_BISHOPS_BB: usize = 8;
pub const BLACK_ROOKS_BB: usize = 9;
pub const BLACK_QUEENS_BB: usize = 10;
pub const BLACK_KINGS_BB: usize = 11;
pub const ALL_PAWNS_BB: usize = 12;
pub const ALL_KNIGHTS_BB: usize = 13;
pub const ALL_BISHOPS_BB: usize = 14;
pub const ALL_ROOKS_BB: usize = 15;
pub const ALL_QUEENS_BB: usize = 16;
pub const ALL_KINGS_BB: usize = 17;
pub const WHITE_PIECES_BB: usize = 18;
pub const BLACK_PIECES_BB: usize = 19;
pub const ALL_PIECES_BB: usize = 20;
pub const EMPTY_SQUARES_BB: usize = 21;

pub const INITIAL_WHITE_PAWNS: BitBoard = BitBoard(65280);
pub const INITIAL_WHITE_KNIGHTS: BitBoard = BitBoard(66);
pub const INITIAL_WHITE_BISHOPS: BitBoard = BitBoard(36);
pub const INITIAL_WHITE_ROOKS: BitBoard = BitBoard(129);
pub const INITIAL_WHITE_QUEENS: BitBoard = BitBoard(8);
pub const INITIAL_WHITE_KINGS: BitBoard = BitBoard(16);
pub const INITIAL_BLACK_PAWNS: BitBoard = BitBoard(71776119061217280);
pub const INITIAL_BLACK_KNIGHTS: BitBoard = BitBoard(4755801206503243776);
pub const INITIAL_BLACK_BISHOPS: BitBoard = BitBoard(2594073385365405696);
pub const INITIAL_BLACK_ROOKS: BitBoard = BitBoard(9295429630892703744);
pub const INITIAL_BLACK_QUEENS: BitBoard = BitBoard(576460752303423488);
pub const INITIAL_BLACK_KINGS: BitBoard = BitBoard(1152921504606846976);

pub const RANK_1: BitBoard = BitBoard(255);
pub const RANK_2: BitBoard = BitBoard(65280);
pub const RANK_3: BitBoard = BitBoard(16711680);
pub const RANK_4: BitBoard = BitBoard(4278190080);
pub const RANK_5: BitBoard = BitBoard(1095216660480);
pub const RANK_6: BitBoard = BitBoard(280375465082880);
pub const RANK_7: BitBoard = BitBoard(71776119061217280);
pub const RANK_8: BitBoard = BitBoard(18374686479671623680);

// pub const CLEAR_RANK_1: BitBoard = BitBoard(!RANK_1.0);
// pub const CLEAR_RANK_2: BitBoard = BitBoard(!RANK_2.0);
// pub const CLEAR_RANK_3: BitBoard = BitBoard(!RANK_3.0);
// pub const CLEAR_RANK_4: BitBoard = BitBoard(!RANK_4.0);
// pub const CLEAR_RANK_5: BitBoard = BitBoard(!RANK_5.0);
// pub const CLEAR_RANK_6: BitBoard = BitBoard(!RANK_6.0);
// pub const CLEAR_RANK_7: BitBoard = BitBoard(!RANK_7.0);
// pub const CLEAR_RANK_8: BitBoard = BitBoard(!RANK_8.0);

// pub const RANK_MASK: [BitBoard; 8] = [
//     RANK_8,
//     RANK_7,
//     RANK_6,
//     RANK_5,
//     RANK_4,
//     RANK_3,
//     RANK_2,
//     RANK_1,
// ];

pub const A_FILE: BitBoard = BitBoard(72340172838076673);
pub const B_FILE: BitBoard = BitBoard(144680345676153346);
pub const C_FILE: BitBoard = BitBoard(289360691352306692);
pub const D_FILE: BitBoard = BitBoard(578721382704613384);
pub const E_FILE: BitBoard = BitBoard(1157442765409226768);
pub const F_FILE: BitBoard = BitBoard(2314885530818453536);
pub const G_FILE: BitBoard = BitBoard(4629771061636907072);
pub const H_FILE: BitBoard = BitBoard(9259542123273814144);

pub const CLEAR_A_FILE: BitBoard = BitBoard(!A_FILE.0);
pub const CLEAR_B_FILE: BitBoard = BitBoard(!B_FILE.0);
// pub const CLEAR_C_FILE: BitBoard = BitBoard(!C_FILE.0);
// pub const CLEAR_D_FILE: BitBoard = BitBoard(!D_FILE.0);
// pub const CLEAR_E_FILE: BitBoard = BitBoard(!E_FILE.0);
// pub const CLEAR_F_FILE: BitBoard = BitBoard(!F_FILE.0);
pub const CLEAR_G_FILE: BitBoard = BitBoard(!G_FILE.0);
pub const CLEAR_H_FILE: BitBoard = BitBoard(!H_FILE.0);

// pub const FILE_MASK: [BitBoard; 8] = [
//     A_FILE,
//     B_FILE,
//     C_FILE,
//     D_FILE,
//     E_FILE,
//     F_FILE,
//     G_FILE,
//     H_FILE,
// ];

pub const A8_SQUARE: BitBoard = BitBoard(A_FILE.0 & RANK_8.0);
pub const B8_SQUARE: BitBoard = BitBoard(B_FILE.0 & RANK_8.0);
pub const C8_SQUARE: BitBoard = BitBoard(C_FILE.0 & RANK_8.0);
pub const D8_SQUARE: BitBoard = BitBoard(D_FILE.0 & RANK_8.0);
pub const E8_SQUARE: BitBoard = BitBoard(E_FILE.0 & RANK_8.0);
pub const F8_SQUARE: BitBoard = BitBoard(F_FILE.0 & RANK_8.0);
pub const G8_SQUARE: BitBoard = BitBoard(G_FILE.0 & RANK_8.0);
pub const H8_SQUARE: BitBoard = BitBoard(H_FILE.0 & RANK_8.0);
pub const A7_SQUARE: BitBoard = BitBoard(A_FILE.0 & RANK_7.0);
pub const B7_SQUARE: BitBoard = BitBoard(B_FILE.0 & RANK_7.0);
pub const C7_SQUARE: BitBoard = BitBoard(C_FILE.0 & RANK_7.0);
pub const D7_SQUARE: BitBoard = BitBoard(D_FILE.0 & RANK_7.0);
pub const E7_SQUARE: BitBoard = BitBoard(E_FILE.0 & RANK_7.0);
pub const F7_SQUARE: BitBoard = BitBoard(F_FILE.0 & RANK_7.0);
pub const G7_SQUARE: BitBoard = BitBoard(G_FILE.0 & RANK_7.0);
pub const H7_SQUARE: BitBoard = BitBoard(H_FILE.0 & RANK_7.0);
pub const A6_SQUARE: BitBoard = BitBoard(A_FILE.0 & RANK_6.0);
pub const B6_SQUARE: BitBoard = BitBoard(B_FILE.0 & RANK_6.0);
pub const C6_SQUARE: BitBoard = BitBoard(C_FILE.0 & RANK_6.0);
pub const D6_SQUARE: BitBoard = BitBoard(D_FILE.0 & RANK_6.0);
pub const E6_SQUARE: BitBoard = BitBoard(E_FILE.0 & RANK_6.0);
pub const F6_SQUARE: BitBoard = BitBoard(F_FILE.0 & RANK_6.0);
pub const G6_SQUARE: BitBoard = BitBoard(G_FILE.0 & RANK_6.0);
pub const H6_SQUARE: BitBoard = BitBoard(H_FILE.0 & RANK_6.0);
pub const A5_SQUARE: BitBoard = BitBoard(A_FILE.0 & RANK_5.0);
pub const B5_SQUARE: BitBoard = BitBoard(B_FILE.0 & RANK_5.0);
pub const C5_SQUARE: BitBoard = BitBoard(C_FILE.0 & RANK_5.0);
pub const D5_SQUARE: BitBoard = BitBoard(D_FILE.0 & RANK_5.0);
pub const E5_SQUARE: BitBoard = BitBoard(E_FILE.0 & RANK_5.0);
pub const F5_SQUARE: BitBoard = BitBoard(F_FILE.0 & RANK_5.0);
pub const G5_SQUARE: BitBoard = BitBoard(G_FILE.0 & RANK_5.0);
pub const H5_SQUARE: BitBoard = BitBoard(H_FILE.0 & RANK_5.0);
pub const A4_SQUARE: BitBoard = BitBoard(A_FILE.0 & RANK_4.0);
pub const B4_SQUARE: BitBoard = BitBoard(B_FILE.0 & RANK_4.0);
pub const C4_SQUARE: BitBoard = BitBoard(C_FILE.0 & RANK_4.0);
pub const D4_SQUARE: BitBoard = BitBoard(D_FILE.0 & RANK_4.0);
pub const E4_SQUARE: BitBoard = BitBoard(E_FILE.0 & RANK_4.0);
pub const F4_SQUARE: BitBoard = BitBoard(F_FILE.0 & RANK_4.0);
pub const G4_SQUARE: BitBoard = BitBoard(G_FILE.0 & RANK_4.0);
pub const H4_SQUARE: BitBoard = BitBoard(H_FILE.0 & RANK_4.0);
pub const A3_SQUARE: BitBoard = BitBoard(A_FILE.0 & RANK_3.0);
pub const B3_SQUARE: BitBoard = BitBoard(B_FILE.0 & RANK_3.0);
pub const C3_SQUARE: BitBoard = BitBoard(C_FILE.0 & RANK_3.0);
pub const D3_SQUARE: BitBoard = BitBoard(D_FILE.0 & RANK_3.0);
pub const E3_SQUARE: BitBoard = BitBoard(E_FILE.0 & RANK_3.0);
pub const F3_SQUARE: BitBoard = BitBoard(F_FILE.0 & RANK_3.0);
pub const G3_SQUARE: BitBoard = BitBoard(G_FILE.0 & RANK_3.0);
pub const H3_SQUARE: BitBoard = BitBoard(H_FILE.0 & RANK_3.0);
pub const A2_SQUARE: BitBoard = BitBoard(A_FILE.0 & RANK_2.0);
pub const B2_SQUARE: BitBoard = BitBoard(B_FILE.0 & RANK_2.0);
pub const C2_SQUARE: BitBoard = BitBoard(C_FILE.0 & RANK_2.0);
pub const D2_SQUARE: BitBoard = BitBoard(D_FILE.0 & RANK_2.0);
pub const E2_SQUARE: BitBoard = BitBoard(E_FILE.0 & RANK_2.0);
pub const F2_SQUARE: BitBoard = BitBoard(F_FILE.0 & RANK_2.0);
pub const G2_SQUARE: BitBoard = BitBoard(G_FILE.0 & RANK_2.0);
pub const H2_SQUARE: BitBoard = BitBoard(H_FILE.0 & RANK_2.0);
pub const A1_SQUARE: BitBoard = BitBoard(A_FILE.0 & RANK_1.0);
pub const B1_SQUARE: BitBoard = BitBoard(B_FILE.0 & RANK_1.0);
pub const C1_SQUARE: BitBoard = BitBoard(C_FILE.0 & RANK_1.0);
pub const D1_SQUARE: BitBoard = BitBoard(D_FILE.0 & RANK_1.0);
pub const E1_SQUARE: BitBoard = BitBoard(E_FILE.0 & RANK_1.0);
pub const F1_SQUARE: BitBoard = BitBoard(F_FILE.0 & RANK_1.0);
pub const G1_SQUARE: BitBoard = BitBoard(G_FILE.0 & RANK_1.0);
pub const H1_SQUARE: BitBoard = BitBoard(H_FILE.0 & RANK_1.0);

pub const SQUARE_MASK: [BitBoard; 64] = [
    A8_SQUARE,
    B8_SQUARE,
    C8_SQUARE,
    D8_SQUARE,
    E8_SQUARE,
    F8_SQUARE,
    G8_SQUARE,
    H8_SQUARE,
    A7_SQUARE,
    B7_SQUARE,
    C7_SQUARE,
    D7_SQUARE,
    E7_SQUARE,
    F7_SQUARE,
    G7_SQUARE,
    H7_SQUARE,
    A6_SQUARE,
    B6_SQUARE,
    C6_SQUARE,
    D6_SQUARE,
    E6_SQUARE,
    F6_SQUARE,
    G6_SQUARE,
    H6_SQUARE,
    A5_SQUARE,
    B5_SQUARE,
    C5_SQUARE,
    D5_SQUARE,
    E5_SQUARE,
    F5_SQUARE,
    G5_SQUARE,
    H5_SQUARE,
    A4_SQUARE,
    B4_SQUARE,
    C4_SQUARE,
    D4_SQUARE,
    E4_SQUARE,
    F4_SQUARE,
    G4_SQUARE,
    H4_SQUARE,
    A3_SQUARE,
    B3_SQUARE,
    C3_SQUARE,
    D3_SQUARE,
    E3_SQUARE,
    F3_SQUARE,
    G3_SQUARE,
    H3_SQUARE,
    A2_SQUARE,
    B2_SQUARE,
    C2_SQUARE,
    D2_SQUARE,
    E2_SQUARE,
    F2_SQUARE,
    G2_SQUARE,
    H2_SQUARE,
    A1_SQUARE,
    B1_SQUARE,
    C1_SQUARE,
    D1_SQUARE,
    E1_SQUARE,
    F1_SQUARE,
    G1_SQUARE,
    H1_SQUARE,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_work() {
        assert_eq!(
            BitBoard::from_str("1111111100000000000000000000000000000000000000000000000000000000"),
            RANK_8,
        );

        assert_eq!(
            BitBoard::from_str("0000000011111111000000000000000000000000000000000000000000000000"),
            RANK_7,
        );

        assert_eq!(
            BitBoard::from_str("0000000000000000111111110000000000000000000000000000000000000000"),
            RANK_6,
        );

        assert_eq!(
            BitBoard::from_str("0000000000000000000000001111111100000000000000000000000000000000"),
            RANK_5,
        );

        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000011111111000000000000000000000000"),
            RANK_4,
        );

        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000111111110000000000000000"),
            RANK_3,
        );

        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000001111111100000000"),
            RANK_2,
        );

        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000000000011111111"),
            RANK_1,
        );
    }

    #[test]
    fn files_work() {
        assert_eq!(
            BitBoard::from_str("1000000010000000100000001000000010000000100000001000000010000000"),
            H_FILE,
        );

        assert_eq!(
            BitBoard::from_str("0100000001000000010000000100000001000000010000000100000001000000"),
            G_FILE,
        );

        assert_eq!(
            BitBoard::from_str("0010000000100000001000000010000000100000001000000010000000100000"),
            F_FILE,
        );

        assert_eq!(
            BitBoard::from_str("0001000000010000000100000001000000010000000100000001000000010000"),
            E_FILE,
        );

        assert_eq!(
            BitBoard::from_str("0000100000001000000010000000100000001000000010000000100000001000"),
            D_FILE,
        );

        assert_eq!(
            BitBoard::from_str("0000010000000100000001000000010000000100000001000000010000000100"),
            C_FILE,
        );

        assert_eq!(
            BitBoard::from_str("0000001000000010000000100000001000000010000000100000001000000010"),
            B_FILE,
        );

        assert_eq!(
            BitBoard::from_str("0000000100000001000000010000000100000001000000010000000100000001"),
            A_FILE,
        );
    }

    #[test]
    fn squares_work() {
        assert_eq!(
            BitBoard::from_str("1000000000000000000000000000000000000000000000000000000000000000"),
            H8_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0100000000000000000000000000000000000000000000000000000000000000"),
            G8_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0010000000000000000000000000000000000000000000000000000000000000"),
            F8_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0001000000000000000000000000000000000000000000000000000000000000"),
            E8_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000100000000000000000000000000000000000000000000000000000000000"),
            D8_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000010000000000000000000000000000000000000000000000000000000000"),
            C8_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000001000000000000000000000000000000000000000000000000000000000"),
            B8_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000100000000000000000000000000000000000000000000000000000000"),
            A8_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000010000000000000000000000000000000000000000000000000000000"),
            H7_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000001000000000000000000000000000000000000000000000000000000"),
            G7_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000100000000000000000000000000000000000000000000000000000"),
            F7_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000010000000000000000000000000000000000000000000000000000"),
            E7_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000001000000000000000000000000000000000000000000000000000"),
            D7_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000100000000000000000000000000000000000000000000000000"),
            C7_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000010000000000000000000000000000000000000000000000000"),
            B7_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000001000000000000000000000000000000000000000000000000"),
            A7_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000100000000000000000000000000000000000000000000000"),
            H6_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000010000000000000000000000000000000000000000000000"),
            G6_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000001000000000000000000000000000000000000000000000"),
            F6_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000100000000000000000000000000000000000000000000"),
            E6_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000010000000000000000000000000000000000000000000"),
            D6_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000001000000000000000000000000000000000000000000"),
            C6_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000100000000000000000000000000000000000000000"),
            B6_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000010000000000000000000000000000000000000000"),
            A6_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000001000000000000000000000000000000000000000"),
            H5_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000100000000000000000000000000000000000000"),
            G5_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000010000000000000000000000000000000000000"),
            F5_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000001000000000000000000000000000000000000"),
            E5_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000100000000000000000000000000000000000"),
            D5_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000010000000000000000000000000000000000"),
            C5_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000001000000000000000000000000000000000"),
            B5_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000100000000000000000000000000000000"),
            A5_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000010000000000000000000000000000000"),
            H4_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000001000000000000000000000000000000"),
            G4_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000100000000000000000000000000000"),
            F4_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000010000000000000000000000000000"),
            E4_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000001000000000000000000000000000"),
            D4_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000100000000000000000000000000"),
            C4_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000010000000000000000000000000"),
            B4_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000001000000000000000000000000"),
            A4_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000100000000000000000000000"),
            H3_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000010000000000000000000000"),
            G3_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000001000000000000000000000"),
            F3_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000100000000000000000000"),
            E3_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000010000000000000000000"),
            D3_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000001000000000000000000"),
            C3_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000100000000000000000"),
            B3_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000010000000000000000"),
            A3_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000001000000000000000"),
            H2_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000100000000000000"),
            G2_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000010000000000000"),
            F2_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000001000000000000"),
            E2_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000000100000000000"),
            D2_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000000010000000000"),
            C2_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000000001000000000"),
            B2_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000000000100000000"),
            A2_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000000000010000000"),
            H1_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000000000001000000"),
            G1_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000000000000100000"),
            F1_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000000000000010000"),
            E1_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000000000000001000"),
            D1_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000000000000000100"),
            C1_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000000000000000010"),
            B1_SQUARE,
        );
        assert_eq!(
            BitBoard::from_str("0000000000000000000000000000000000000000000000000000000000000001"),
            A1_SQUARE,
        );
    }
}