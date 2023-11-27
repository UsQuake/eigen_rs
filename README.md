# Rust로 고유값/고유벡터 찾기

  ## 소개
  
   - Rust로 간단한 iterative power method 구현해 고유값/고유벡터 찾기.(Describe how to get eigenvalue/eigenvector with iterative power method in rust.)
     
  ## 선행 지식

   - 선형대수학에 대한 이해.
   - Rust/C++에 대한 이해.
     
  ## 요구 사항
   
   - Rust 버전 ***1.74.0***
   - 참고: *Rust Installation*[https://rinthel.github.io/rust-lang-book-ko/ch01-01-installation.html]

  ## 구현 과정
  
  ### 1.행렬/열벡터를 정의합니다.

  - 인덱스를 통해 행렬에[행][열]꼴로 접근할 수 있게 다음과 같이 정의합니다.
    ```Rust
        struct Matrix<const ROW_COUNT: usize, const COLUMN_COUNT: usize>{
        elements: [[f64;COLUMN_COUNT];ROW_COUNT]
        }
    ```
  - 요소가 2개, 3개인 열벡터를 행렬을 통해 정의합니다.
    ```Rust
    type Vec3 = Matrix<3, 1>;
    type Vec2 = Matrix<2, 1>;
    ```
  ### 2.벡터/행렬에 대한 기본적인 연산들(정규화, 내적, 행렬곱)을 구현합니다.

  - 행렬과 열벡터의 곱
    ```Rust
    fn mul(self, other_vector: Vec3) -> Vec3{
       Matrix{
        elements :[
                  [ self.elements[0][0] * other_vector.elements[0][0] + self.elements[0][1] * other_vector.elements[1][0] + self.elements[0][2] * other_vector.elements[2][0] ],
                  [ self.elements[1][0] * other_vector.elements[0][0] + self.elements[1][1] * other_vector.elements[1][0] + self.elements[1][2] * other_vector.elements[2][0] ],
                  [ self.elements[2][0] * other_vector.elements[0][0] + self.elements[2][1] * other_vector.elements[1][0] + self.elements[2][2] * other_vector.elements[2][0] ]
               ]
         }
    }
    ```
  - 벡터 간의 내적
    ```Rust
    pub fn dot(&self, other: Matrix<ROW_COUNT, 1>) -> f64{
        let mut sum = 0.0;
        for i in 0..ROW_COUNT{
            sum += self.elements[i][0] * other.elements[i][0];
        }
        sum
    }
    ```
  - 벡터 정규화
    ```Rust
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
    ```

  ### 3.iterative power method를 구현합니다.

  - 
      ```Rust
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
      ```
