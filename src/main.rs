#[derive(Clone, Copy, Debug)]
struct Matrix<const ROW_COUNT: usize, const COLUMN_COUNT: usize>{
    elements: [[f64;COLUMN_COUNT];ROW_COUNT]
}
impl<const N: usize> Matrix<N, N>
{
    pub fn get_identity_matrix()-> Self{
        let mut result = Self{elements: [[0.0;N];N]};
        for i in 0..N{
            result.elements[i][i] = 1.0;
        }
        result
    }
    pub fn get_inverse_matrix(&self) -> Self
    {
       let mut self_copy = self.clone(); 
       let mut result =  Self::get_identity_matrix();
       for i in 0..N{
        let pivot = self_copy.elements[i][i];
        for j  in 0..N{
            self_copy.elements[i][j] /= pivot;
            result.elements[i][j] /= pivot;
        }


        for j in 0..i{   
            let ratio = self_copy.elements[j][i];

            for k in 0..N{   
                    result.elements[j][k] -= ratio * result.elements[i][k];
                    self_copy.elements[j][k] -= ratio * self_copy.elements[i][k];
            }
        }
        for j in i + 1..N{
            let ratio = self_copy.elements[j][i];

            for k in 0..N{
                result.elements[j][k] -= ratio * result.elements[i][k];
                self_copy.elements[j][k] -= ratio *  self_copy.elements[i][k];
            }
        }
       }
       result
    }

    pub fn get_dominant_eigen(&self, x: Matrix<N,1>, try_count: usize) -> (Matrix<N,1>, Matrix<N,1>, f64)
    {
        let mut try_count = try_count;
        let mut x= x.clone();
        let mat_a = self.clone(); 
        loop{
            let a_mul_x = mat_a * x;
            if try_count == 0 
            {
                return (x, a_mul_x, a_mul_x.get_abs_max());
            }
            x = a_mul_x / a_mul_x.get_abs_max();
            try_count -= 1;
        }

    }

    pub fn get_alpha_nearest_eigen(&self, x: Matrix<N,1>, try_count: usize, alpha:f64)-> (Matrix<N,1>, Matrix<N,1>, f64, f64){
        let mut try_count = try_count;
        let mut x= x.clone();
        let solution_with_matrix: Matrix<N, N> = (*self -  Self::get_identity_matrix() * alpha).get_inverse_matrix(); 
        loop{
            let y = solution_with_matrix * x;
            let mu =  y.get_abs_max();
            let v = alpha + (1.0 / mu);
            if try_count == 0 
            {
                return (x, y, mu, v);
            }
            x = y / mu;
            try_count -= 1;
        }

    }

    pub fn get_dominant_eigen_auto(&self, x: Matrix<N,1>) -> (Matrix<N,1>, Matrix<N,1>, f64, usize)
    {
        let mut x= x.clone();
        let mat_a = self.clone(); 
        let mut mu = 0.0;
        let mut try_count = 0;
        loop{
            let a_mul_x = mat_a * x;
            let prev_mu = mu;
            mu = a_mul_x.get_abs_max();
            if (mu - prev_mu).abs() < std::f64::EPSILON
            {
                return (x, a_mul_x, mu, try_count);
            }
            x = a_mul_x / mu;
           try_count += 1;
        }

    }

    pub fn get_alpha_nearest_eigen_auto(&self, x: Matrix<N,1>, alpha: f64)-> (Matrix<N,1>, Matrix<N,1>, f64, f64, usize){
        let mut try_count = 0;
        let mut x= x.clone();
        let mut v = 0.0;
        let solution_with_matrix: Matrix<N, N> = (*self -  Self::get_identity_matrix() * alpha).get_inverse_matrix(); 
        loop{
            let y = solution_with_matrix * x;
            let mu =  y.get_abs_max();
            let prev_v = v;
            v = alpha + (1.0 / mu);
            if (v - prev_v).abs() < std::f64::EPSILON
            {
                return (x, y, mu, v, try_count);
            }
            x = y / mu;
            try_count += 1;
        }
    

    }
}
impl<const ROW_COUNT: usize, const COLUMN_COUNT: usize> std::ops::Div<f64> for Matrix<ROW_COUNT,COLUMN_COUNT>{
    type Output = Matrix<ROW_COUNT,COLUMN_COUNT>;
    fn div(self, other_scala:f64) -> Matrix<ROW_COUNT,COLUMN_COUNT>{
        let mut result = Matrix{elements:[[0.0;COLUMN_COUNT];ROW_COUNT]};
        for i in 0..ROW_COUNT{
            for j in 0..COLUMN_COUNT{
                result.elements[i][j] = self.elements[i][j] / other_scala;
            }
        }
        result
    }
}

impl<const ROW_COUNT: usize, const COLUMN_COUNT: usize> std::ops::Sub<Matrix<ROW_COUNT,COLUMN_COUNT>> for Matrix<ROW_COUNT,COLUMN_COUNT>{
    type Output = Matrix<ROW_COUNT,COLUMN_COUNT>;
    fn sub(self, other_matrix: Self) -> Matrix<ROW_COUNT,COLUMN_COUNT>{
        let mut result = Matrix{elements:[[0.0;COLUMN_COUNT];ROW_COUNT]};
        for i in 0..ROW_COUNT{
            for j in 0..COLUMN_COUNT{
                result.elements[i][j] = self.elements[i][j] - other_matrix.elements[i][j];
            }
        }
        result
    }
}

impl<const ROW_COUNT: usize> Matrix<ROW_COUNT, 1>
{
    pub fn get_abs_max(&self) -> f64{
        let mut max:f64 = 0.0;
        for i in self.elements{
            if max.abs() < i[0].abs() {
                max = i[0];
            }
        }
        max
    }
}

impl<const ROW_COUNT: usize, const COLUMN_COUNT: usize> std::ops::Mul<f64> for Matrix<ROW_COUNT,COLUMN_COUNT>
{
    type Output = Matrix<ROW_COUNT,COLUMN_COUNT>;

    fn mul(self, other_scala: f64) -> Matrix<ROW_COUNT,COLUMN_COUNT>{
        let mut result = Matrix{elements: [[0.0;COLUMN_COUNT];ROW_COUNT]};
        for row in 0..ROW_COUNT{
            for col in 0..COLUMN_COUNT{
                result.elements[row][col] = self.elements[row][col] * other_scala;
            }
        }
        result
    }
}

impl<const ROW_COUNT: usize, const COLUMN_COUNT: usize> std::ops::Mul<Matrix<COLUMN_COUNT,1>> for Matrix<ROW_COUNT,COLUMN_COUNT>
{
    type Output = Matrix<ROW_COUNT,1>;

    fn mul(self, other_vector: Matrix<COLUMN_COUNT,1>) -> Matrix<ROW_COUNT,1>{
        let mut result = Matrix{elements: [[0.0;1];ROW_COUNT]};
        for row in 0..ROW_COUNT{
            for col in 0..COLUMN_COUNT{
                result.elements[row][0] += self.elements[row][col] * other_vector.elements[col][0];
            }
        }
        result
    }
}
impl<const COLUMN_COUNT: usize>  std::fmt::Display for Matrix<COLUMN_COUNT,1> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        write!(f,"[")?;
        for i in 0..COLUMN_COUNT - 1{
            write!(f,"{:.4}, ",self.elements[i][0])?;
        }
        write!(f,"{:.4}]",self.elements[COLUMN_COUNT - 1][0])?;
        Ok(())
    }
}



fn main() {
    let mat_a = Matrix{elements: [[6.0, 5.0],
                                             [1.0, 2.0]]};
    let x = Matrix{elements: [[0.0], [1.0]]};
    let try_count = 6;

    for i in 0..try_count{
        let problem1_solution = mat_a.get_dominant_eigen(x, i);

        println!("1번 문제: {i}번째 실행 결과");
        println!("xk: {}", problem1_solution.0);
        println!("A x xk: {}", problem1_solution.1);
        println!("μk: {:.4}", problem1_solution.2);
        println!("");
    }

    {
        let problem1_solution = mat_a.get_dominant_eigen_auto(x);

        println!("1번 문제: {}번째 실행 결과",problem1_solution.3);
        println!("xk: {}", problem1_solution.0);
        println!("A x xk: {}", problem1_solution.1);
        println!("μk: {:.4}", problem1_solution.2);
        println!("");
    }



    let mat_b = Matrix{elements: [[10.0, -8.0, -4.0],
                                            [-8.0, -13.0, 4.0],
                                            [-4.0, 5.0, 4.0]]};
    let x0 = Matrix{elements: [[1.0], [1.0], [1.0]]};
    for i in 0..try_count{
        let problem2_solution = mat_b.get_alpha_nearest_eigen(x0, i, 21.0);
        println!("2번 문제(21과 가까운 eigen): {i}번째 실행 결과");
        println!("xk: {}", problem2_solution.0);
        println!("yk: {}", problem2_solution.1);
        println!("μk: {:.4}", problem2_solution.2);
        println!("vk: {:.4}", problem2_solution.3);
        println!("");
    }

    for i in 0..try_count{
        let problem2_solution = mat_b.get_alpha_nearest_eigen(x0, i, 1.9);
        println!("2번 문제(3.3과 가까운 eigen): {i}번째 실행 결과");
        println!("xk: {}", problem2_solution.0);
        println!("yk: {}", problem2_solution.1);
        println!("μk: {:.4}", problem2_solution.2);
        println!("vk: {:.4}", problem2_solution.3);
        println!("");
    }

    for i in 0..try_count{
        let problem2_solution = mat_b.get_alpha_nearest_eigen(x0, i, 3.3);
        println!("2번 문제(1.9와 가까운 eigen): {i}번째 실행 결과");
        println!("xk: {}", problem2_solution.0);
        println!("yk: {}", problem2_solution.1);
        println!("μk: {:.4}", problem2_solution.2);
        println!("vk: {:.4}", problem2_solution.3);
        println!("");
    }


    let problem2_solution = mat_b.get_alpha_nearest_eigen_auto(x0, 21.0);
    println!("2번 문제(21과 가까운 eigen): {}번째 실행 결과", problem2_solution.4);
    println!("xk: {}", problem2_solution.0);
    println!("yk: {}", problem2_solution.1);
    println!("μk: {:.4}", problem2_solution.2);
    println!("vk: {:.4}", problem2_solution.3);
    println!("");
}
