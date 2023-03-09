FROM python:3.10

WORKDIR /codegen
COPY . .
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

WORKDIR /codegen/ui
RUN pip install poetry
RUN poetry install

CMD ["poetry", "run", "python", "app.py", "--host", "0.0.0.0"]
