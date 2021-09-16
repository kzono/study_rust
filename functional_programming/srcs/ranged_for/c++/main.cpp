#include <iostream>
#include <vector>

int main(){
    std::vector<int> v{1, 2, 3};

    // 範囲 for文
    for(const auto& e: v){
        std::cout << e << std::endl;
    }

    std::cout << std::endl;

    // 普通のfor文
    for(int i = 0; i < v.size(); i++){
        std::cout << v[i] << std::endl;
    }
}