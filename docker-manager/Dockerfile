FROM python:3.10
WORKDIR /code
RUN python3 -m venv /code/venv
COPY ./ /code
RUN pip install --no-cache-dir --upgrade -r /code/requirements.txt
EXPOSE 7588
CMD ["uvicorn", "src.main:app", "--host", "0.0.0.0", "--port", "7588"]
