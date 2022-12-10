from pydantic import BaseModel

# Create Message Schema (Pydantic Model)
class MessageCreate(BaseModel):
    text: str
    user: str
    timestamp: int

# Complete Message Schema (Pydantic Model)

class Message(BaseModel):
    id: int
    text: str
    user: str
    timestamp: int

    class Config:
        orm_mode = True