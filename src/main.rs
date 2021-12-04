mod day_1;
mod day_2;
mod day_3;
mod util;

fn main() {
    //day_1::count_measurement_increases(util::read_resource("day_1.txt"));
    //day_1::count_measurement_increases_sliding(util::read_resource("day_1.txt"));
    //day_2::count_sub_position(util::read_resource("day_2.txt"));
    //day_2::count_sub_position_with_aim(util::read_resource("day_2.txt"));
    day_3::calculate_power_consumption(util::read_resource("day_3.txt"));
}
