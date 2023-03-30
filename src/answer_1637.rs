pub mod answer_1637{
    use std::cmp::max;

    pub fn get_answer(points: Vec<Vec<i32>>) -> i32 {
        let mut pointss = Vec::new();
        for point in points{
            pointss.push(point[0]);
        }
        pointss.sort();
        let mut max_:i32 = 0;
        let mut last_ = pointss[0];
        for point in pointss{
            max_ = max(point - last_, max_);
            last_ = point.clone();
        }
        max_
    }
}