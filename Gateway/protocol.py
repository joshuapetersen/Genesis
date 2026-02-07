from pydantic import BaseModel, Field
from typing import Optional, Dict, Any, List

class RequestFrame(BaseModel):
    id: str
    method: str
    params: Optional[Dict[str, Any]] = Field(default_factory=dict)
    type: str = "req"

class ResponseFrame(BaseModel):
    id: str
    ok: bool
    payload: Optional[Dict[str, Any]] = None
    error: Optional[str] = None
    type: str = "res"

class EventFrame(BaseModel):
    event: str
    payload: Dict[str, Any]
    type: str = "event"
    seq: Optional[int] = None
