from sqlalchemy import Column, Integer, String
from database import Base

# Define To Do class inheriting from Base

class Message(Base):
    __tablename__ = 'messages'
    id = Column(Integer, primary_key=True)
    text = Column(String(1000))
    user = Column(String(256))
    timestamp = Column(Integer)