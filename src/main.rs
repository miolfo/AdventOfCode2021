mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod util;

fn main() {
    //day_1::count_measurement_increases(util::read_resource("day_1.txt"));
    //day_1::count_measurement_increases_sliding(util::read_resource("day_1.txt"));
    //day_2::count_sub_position(util::read_resource("day_2.txt"));
    //day_2::count_sub_position_with_aim(util::read_resource("day_2.txt"));
    //day_3::calculate_power_consumption(util::read_resource("day_3.txt"));
    //day_3::calculate_life_support_rating(util::read_resource("day_3.txt"));
    //day_4::count_bingo_wins(util::read_resource("day_4.txt"));
    //day_4::count_last_to_win(util::read_resource("day_4.txt"));
    day_5::count_dangerous_points(util::read_resource("day_5.txt"))
}
