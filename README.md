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
  
  ### 1. 각종 선형대수학 함수들을 구현합니다. 과제를 위해 일부만 구현했습니다.(Implement well-known linear-algebra functions.)
  - 요소가 2개, 3개인 열벡터를 행렬을 통해 정의
    ```Rust
    type Vec3 = Matrix<3, 1>;
    type Vec2 = Matrix<2, 1>;
    ```
  - 행렬 x 열벡터
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
