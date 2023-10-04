import sys
import openai
import subprocess
from rich.panel import Panel
from rich.console import Console
openai.api_key = ''
openai.Model.list()
console = Console()

option_style = "bold cyan"
success_style = "bold green"

file = open(sys.argv[1])
content = file.read()

console.print(f'[{option_style}]Carregando...')

response = openai.Completion.create(
  model="gpt-3.5-turbo-instruct",
  prompt=f"Faça uma descrição em português-br para um pull request do código {content}",
  max_tokens=500,
  temperature=0.9
)

title_pr = 'Added file: ' + sys.argv[1]
descricao_pr = response['choices'][0]['text']

command = ['gh', 'pr', 'create', '--title', title_pr, '--body', descricao_pr]

result = subprocess.run(command, capture_output=True, text=True)

if result.returncode != 0:
    console.print(f'[{option_style}]Erro ao executar comando 😴')
else:
    console.print(f'[{success_style}]Descrição para o pull request gerada com sucesso! 🥱')
