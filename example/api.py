from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse
import jwt

app = FastAPI()


@app.middleware("http")
async def verify_jwt(request: Request, call_next):
    if request.url.path == "/":
        return await call_next(request)

    # Extract Authorization header
    authorization = request.headers.get("Authorization")
    if authorization is None:
        return JSONResponse(content={"message": "Authorization header is missing"}, status_code=401)

    try:
        scheme, token = authorization.split()
        if not token:
            return JSONResponse(content={"message": "Token is missing"}, status_code=401)

        decoded = jwt.decode(token, "test", algorithms=["HS256"])
        print(decoded)
    except jwt.PyJWTError as e:
        return JSONResponse(content={"message": str(e)}, status_code=401)
    except ValueError:
        return JSONResponse(content={"message": "Invalid Authorization header format"}, status_code=400)

    return await call_next(request)


@app.get("/protected")
async def protected():
    return JSONResponse(content={"message": "This is a protected route"}, status_code=200)


@app.get("/")
async def root():
    return JSONResponse(content={"message": "Hello"}, status_code=200)
