from typing import List
from fastapi import FastAPI, status, HTTPException, Depends
from database import Base, engine, SessionLocal
from sqlalchemy.orm import Session
import models
import schemas

# Create the database
Base.metadata.create_all(engine)

# Initialize app
app = FastAPI()

# Helper function to get database session

def get_session():
    session = SessionLocal()
    try:
        yield session
    finally:
        session.close()

@app.get("/")
def root():
    return {"message": "Hello World"}

@app.post("/message", response_model=schemas.Message, status_code = status.HTTP_201_CREATED)
def create_message(message: schemas.MessageCreate, session = Depends(get_session)):

    # Check if request is valid 
    if not message.user or not message.text:
        raise HTTPException(status_code=400, detail=f"Request didn't contain user or text")

    # Models the input as an object
    messagedb = models.Message(text = message.text, user = message.user)

    # Commits the object to the db
    session.add(messagedb)
    session.commit()
    session.refresh(messagedb)
    
    return messagedb

@app.get("/message/{id}", response_model=schemas.Message)
def read_message(id: int, session: Session = Depends(get_session)):

    message = session.query(models.Message).get(id)

    if not message:
        raise HTTPException(status_code=404, detail=f"message with id {id} not found")

    return message

@app.delete("/message/{id}", response_model=schemas.Message)
def delete_message(id: int, session: Session = Depends(get_session)):
    
    message = session.query(models.Message).get(id)

    if message:
        session.delete(message)
        session.commit()
    else:
        raise HTTPException(status_code=404, detail=f"message with id {id} not found")

    return None

@app.get("/message", response_model=List[schemas.Message])
def read_message(session: Session = Depends(get_session)):

    message_list = session.query(models.Message).all()

    return message_list

# 