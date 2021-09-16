class Main{
    public static void main(String[] args){
        int[] v = {1, 2, 3};

        // 普通のfor文
        for(int i = 0; i < v.length; i++){
            System.out.println(v[i]);
        }
        System.out.println();

        // 拡張for文
        for(int e: v){
            System.out.println(e);
        }
    }
}