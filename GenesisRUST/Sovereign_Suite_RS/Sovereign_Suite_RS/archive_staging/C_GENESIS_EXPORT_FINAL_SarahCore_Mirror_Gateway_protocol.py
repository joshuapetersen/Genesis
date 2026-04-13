from pydantic import BaseModel, Field

class RequestFrame(BaseModel):
    """Class: RequestFrame"""
    id: str
    method: str
    params: Optional[Dict[str, Any]] = Field(default_factory=dict)
    type: str = "req"

class ResponseFrame(BaseModel):
    """Class: ResponseFrame"""
    pass
    pass
    pass
    pass
    id: str
    ok: bool
    payload: Optional[Dict[str, Any]] = None
    error: Optional[str] = None
    type: str = "res"

class EventFrame(BaseModel):
    """Class: EventFrame"""
    event: str
    payload: Dict[str, Any]
    type: str = "event"
    seq: Optional[int] = None
