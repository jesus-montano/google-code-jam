import java.util.Scanner;
public class TroubleSort{
public static void main(String[] args) {
    Scanner teclado = new Scanner(System.in);
    int T=-1;
    int N=0;
    while(T<1 ||T>20){
        System.out.println("ingrese cantidad de casos de prueba entre 1 y 20");
        T=teclado.nextInt();}
        while(N<3 || N>Math.pow(10,9)){
        System.out.println("ingrese cantidad de numeros en su arreglo");
        N=teclado.nextInt();}
        int[] V= new int [N];
        for (int i=0; i<N; i++) {
            System.out.println("ingrese numero "+i);
            V[i]=teclado.nextInt();
        }
        int [] o=sort(V);
        check(o);
}
public static int[] sort(int[] v){
    boolean done=false;
    while(!done){
        done=true;
        for(int i=0; i<v.length-2; i++){
        
            if(v[i]>v[i+2]){
            
                done=false; 
                int aux;
                aux=v[i];
                v[i]=v[i+2];
                v[i+2]= aux;
            }
        }
    }
    return v;
    }
    public static void check(int[] o){
        boolean flag=false;
        for(int i=0; i<o.length-1; i++){
            if(o[i]>o[i+1]){
            System.out.println(i);
            flag=true;
            break; 
            }
        }
        if(!flag){
        System.out.println("ok");
        }
    
    }
}