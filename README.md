# Rust로 고유값/고유벡터 찾기

  ## Introduction
  
   - Rust로 간단한 iterative power method 구현해 고유값/고유벡터 찾기.(Describe how to get eigenvalue/eigenvector with iterative power method in rust.)
     
  ## Acknowledge

   - 선형대수학에 대한 이해(Knowledge about linear algebra).
   - Rust/C++에 대한 이해(Knowledege about Rust or C++ optimization).
     
  ## Setup & Requirements
   
   - Rust version ***1.74.0***
   - 참고: *Rust Installation*[https://rinthel.github.io/rust-lang-book-ko/ch01-01-installation.html]

  ## Explanation
  
  ### 1.행렬/열벡터를 정의합니다. (1.Define matrix & column vector.)

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
  ### 2.벡터/행렬에 대한 정규화, 내적, 행렬곱을 구현합니다. (2.Implement dot(), normalize(), mul())

  - 행렬과 열벡터의 곱(Implementation of **matrix X column_vector**)
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

  ### 3. 각종 함수를 조합해 고유값/고유벡터를 구합니다.lambda: dot(x, A * x)
