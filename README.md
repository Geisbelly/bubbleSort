<img width=100% src="https://capsule-render.vercel.app/api?type=waving&color=87CEEB&height=120&section=header"/>

[![Typing SVG](https://readme-typing-svg.herokuapp.com/?color=51A6F1&size=40&center=true&vCenter=true&width=1000&lines=Olá,+seja+bem+vindo(a)!;Ao+nosso+projeto!+:%29)](https://git.io/typing-svg)

<div align="center" />

<br>

# The Bubble Sort

  
#### Nesse repositorios disponibilizamos uma implementação do algoritmos de ordenação e busca - Bubble Sort, com adicição de alguma informações sobre o mesmo:
</div> 

<div align="center" />
<br>

## Integrantes

  
<p align="center" class="github-links">
<a href="https://github.com/Geisbelly">
    <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/Geis.png" width="210px" alt="Geisbelly"></a>
    <img src="https://github.com/Geisbelly-vic/Curso-de-Figma-2024-02/blob/main/Imagens/Integrante/Rectangle%20375.png" width="80px">
<a href="https://github.com/baskerbyte">
    <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/Luis.png" width="210px" alt="Luis Fernando"></a>
    <img src="https://github.com/Geisbelly-vic/Curso-de-Figma-2024-02/blob/main/Imagens/Integrante/Rectangle%20375.png" width="80px">
<a href="https://github.com/MariAntonia-010">
    <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/Mary.png" width="210px" alt="Maria Antônia"></a>

#### Geisbelly Victória dos S. F. Moraes  •  Luis Fernando Borges Lima  •  Maria Antônica Alves Curcino
</p>

<br>

  
## Detalhes


</div>
<div align="left" />

> ### Ficha Técnica
> Linguagem: Rust;
> <br>Metodo: Bubble Sort;
> <br>Operação Fundamental: 𝑛2 − 𝑛
> <br>Lógica:
> <br>𝑛 ← 𝐓𝐚𝐦𝐚𝐧𝐡𝐨(𝐿)
><br>para 𝑗 ← 1 até 𝑛 faça
><br>para 𝑖 ← 0 até 𝑛 − 2 faça
><br>se 𝐿𝑖 > 𝐿𝑖+1 então
><br>𝑡 ← 𝐿𝑖
><br>𝐿𝑖 ← 𝐿𝑖+1
><br>𝐿𝑖+1 ← 𝑡
><br>fim-se
><br>fim-para
><br>fim-para

</div>


> ### Código
> 
>``` Rust  
>let n = list.len();
> for _ in 1..(n + 1) {
>      for i in 0..((n + 1) - 2) {
>          if list[i] > list[i + 1] {
>              let t = list[i];
>              list[i] = list[i + 1];
>              list[i + 1] = t
>          }
>      }
>}
>```    


---


<div align="center" />
<br>

## Resultados


### Analise para 10 mil

#### Tempo de execução total: 5 segundos
#### Operação Fundamental: 99.990.000

<div style="display: flex; align-items: flex-start; background-color: #87CEEB; padding: 20px;">
  <img src="https://github.com/Geisbelly/bubbleSort/blob/main/fotos/CPU10k.jpeg" style="margin-right: 50px;">
  <img src="https://github.com/Geisbelly/bubbleSort/blob/main/fotos/RAm10k.jpeg">
</div>
</div>

> #### Legenda:
> Eixo x = Variação do tempo em segundos

<br>

<div align="center">

### Analise para 100 mil

#### Tempo de execução total: 3 minutos e 20 segundos
#### Operação Fundamental: 9.999.900.000

<div style="display: flex; align-items: flex-start; background-color: #87CEEB; padding: 20px;">
  <img src="https://github.com/Geisbelly/bubbleSort/blob/main/fotos/CPU100k.jpeg" style="margin-right: 50px;">
  <img src="https://github.com/Geisbelly/bubbleSort/blob/main/fotos/RAm100k.jpeg">
</div>
</div>

> #### Legenda:
> Eixo x = Variação do tempo em minutos





<img width=100% src="https://capsule-render.vercel.app/api?type=waving&color=87CEEB&height=120&section=footer"/>
