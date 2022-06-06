export function shuffle(array: any[]): any[] {

    let current_index: number = array.length;
    let random_index: number;
    let tmp_value: any;

    // While there remain elements to shuffle
    while (current_index != 0) {

        // Pick a remaining element.
        random_index = Math.floor(Math.random() * current_index);
        current_index--;

        // And swap it with the current element
        tmp_value = array[current_index];
        array[current_index] = array[random_index];
        array[random_index] = tmp_value;
    }

    return array;

}

export function get_board_index(board_position: [number, number]): number {
    return 7 * board_position[0] + board_position[1];

}