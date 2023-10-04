import os
import sys
import openai
openai.api_key = ''
openai.Model.list()

arquivo = open(sys.argv[1])

conteudo = arquivo.read()

response = openai.Completion.create(
  model="gpt-3.5-turbo-instruct",
  prompt=f"Faça a geração de uma descrição de um pull request do código {conteudo}",
  max_tokens=500,
  temperature=0.9
)

descricao_pr = response['choices'][0]['text']
