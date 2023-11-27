#[derive(Clone, Copy, Debug)]
struct Matrix<const ROW_COUNT: usize, const COLUMN_COUNT: usize>{
    elements: [[f64;COLUMN_COUNT];ROW_COUNT]
}
type Vec3 = Matrix<3, 1>;
type Vec2 = Matrix<2, 1>;
impl<const ROW_COUNT: usize> Matrix<ROW_COUNT, 1>
{
    pub fn normalize(&self) -> Matrix<ROW_COUNT, 1>{
        let mut sum: f64 = 0.0;
        for i in self.elements{
            sum += i[0] * i[0];
        }
        let norm = sum.sqrt();
        let mut result = Self { elements: [[0.0;1]; ROW_COUNT] };
        for j in 0..ROW_COUNT{
               result.elements[j][0] = self.elements[j][0] / norm;
        }
        result
    }

    pub fn dot(&self, other: Matrix<ROW_COUNT, 1>) -> f64{
        let mut sum = 0.0;
        for i in 0..ROW_COUNT{
            sum += self.elements[i][0] * other.elements[i][0];
        }
        sum
    }
}

impl std::ops::Mul<Vec2> for Matrix<2,2>
{
    type Output = Vec2;

    fn mul(self, other_vector: Vec2) -> Vec2{
        Matrix{
            elements :[
                [ self.elements[0][0] * other_vector.elements[0][0] + self.elements[0][1] * other_vector.elements[1][0] ],
                [ self.elements[1][0] * other_vector.elements[0][0] + self.elements[1][1] * other_vector.elements[1][0] ]
            ]
        }
    }
}

impl std::ops::Mul<Vec3> for Matrix<3,3>
{
    type Output = Vec3;

    fn mul(self, other_vector: Vec3) -> Vec3{
        Matrix{
            elements :[
                [ self.elements[0][0] * other_vector.elements[0][0] + self.elements[0][1] * other_vector.elements[1][0] + self.elements[0][2] * other_vector.elements[2][0] ],
                [ self.elements[1][0] * other_vector.elements[0][0] + self.elements[1][1] * other_vector.elements[1][0] + self.elements[1][2] * other_vector.elements[2][0] ],
                [ self.elements[2][0] * other_vector.elements[0][0] + self.elements[2][1] * other_vector.elements[1][0] + self.elements[2][2] * other_vector.elements[2][0] ]
            ]
        }
    }
}
impl<const ROW_COUNT: usize>  std::fmt::Display for Matrix<ROW_COUNT, 1> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"[").unwrap();
        for i in 0..ROW_COUNT - 1{
            write!(f,"{:.6}, ",self.elements[i][0]).unwrap();
        }
        write!(f,"{:.6}]",self.elements[ROW_COUNT - 1][0])
    }
}
fn main() {
    let A = Matrix{elements: [[10.0, -8.0, -4.0],
                                             [-8.0, 13.0, 4.0], 
                                             [-4.0, 5.0, 4.0]]};
    let mut x = Vec3{elements: [[1.0], [1.0], [1.0]]};
    let mut lambda = 0.0;
    let mut index = 0;
    loop{
        let Ax: Vec3 = A * x;
        let previous_lambda = lambda;

        println!("{index}번째 실행 결과");
        println!("A x xk: {Ax}");
        println!("λk: {lambda:.6}");
        println!("Vk: {x}\n"); 
        lambda = Ax.dot(x);
        if (lambda - previous_lambda).abs() < std::f64::EPSILON
        {
            break;
        }
        x = Ax.normalize();
        index += 1;
    }
}
