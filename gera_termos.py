import random

NUM_TERMOS = 10000     
MAX_PROFUNDIDADE = 4      

FUNCOES = {
    'f': 2,  # 
    'g': 1,
    'h': 3
}

CONSTANTES = ['a', 'b', 'c', 'd']

def gerar_termo(profundidade=0):
    # condição de parada (folha): sorteia uma constante
    if profundidade >= MAX_PROFUNDIDADE or random.random() < 0.3:
        return [random.choice(CONSTANTES)]
    else:
        func = random.choice(list(FUNCOES.keys()))
        aridade = FUNCOES[func]
        termo = [func]
        for _ in range(aridade):
            termo += gerar_termo(profundidade + 1)
        return termo

def termo_para_str(termo):
    return ' '.join(termo)

def salvar_termos_em_arquivo(nome_arquivo, termos):
    with open(nome_arquivo, 'w') as f:
        for t in termos:
            f.write(termo_para_str(t) + '\n')

# Gerar termos
termos = [gerar_termo() for _ in range(NUM_TERMOS)]

# Salvar em arquivo
salvar_termos_em_arquivo("termos.txt", termos)

print(f"{NUM_TERMOS} termos salvos em termos_constantes.txt")
