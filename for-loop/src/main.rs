// ANCHOR: transpose
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    // ANCHOR_END: transpose
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
	for j in 0..3 {
	    result[j][i] = matrix[i][j];
	}
    }
    return result;
}

// ANCHOR: pretty_print
fn pretty_print(matrix: &[[i32; 3]; 3]) {
    // ANCHOR_END: pretty_print
    // for row in matrix.into_iter() {
    //	println!("{row:?}");
    // }
    for row in matrix.iter() {
	println!("{row:?}")
    }
    println!("matrix:{matrix:?}")
}

// ANCHOR: tests
#[test]
fn test_transpose() {
    // let matrix = [
    //	[101, 102, 103], //
    //	[201, 202, 203],
    //	[301, 302, 303],
    // ];
    let matrix = [
	[1, 2, 3], //
	[4, 5, 6],
	[7, 8, 9],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
	transposed,
	[
	    [101, 201, 301], //
	    [102, 202, 302],
	    [103, 203, 303],
	]
    );
}
// ANCHOR_END: tests

fn main() {
    let matrix = [
	[101, 102, 103], // <-- the comment makes rustfmt add a newline
	[201, 202, 203],
	[301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
