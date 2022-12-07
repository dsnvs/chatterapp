from pydantic import BaseModel

# Create Message Schema (Pydantic Model)
class MessageCreate(BaseModel):
    text: str
    user: str

# Complete Message Schema (Pydantic Model)

class Message(BaseModel):
    id: int
    text: str
    user: str

    class Config:
        orm_mode = True