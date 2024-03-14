from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from sqlalchemy import create_engine, Column, Integer, String
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

# Define database connection
SQLALCHEMY_DATABASE_URL = "sqlite:///./test.db"
engine = create_engine(SQLALCHEMY_DATABASE_URL)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

# Create a base class for ORM models
Base = declarative_base()


# Define Product model
class Product(Base):
    __tablename__ = "products"

    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, index=True)
    description = Column(String, index=True)
    price = Column(Integer)


# Create database tables
Base.metadata.create_all(bind=engine)

# FastAPI instance
app = FastAPI()


# Pydantic model for product creation
class ProductCreate(BaseModel):
    name: str
    description: str
    price: int


# Endpoint to create a new product
@app.post("/products/")
async def create_product(product: ProductCreate):
    db = SessionLocal()
    db_product = Product(**product.dict())
    db.add(db_product)
    db.commit()
    db.refresh(db_product)
    return db_product


# Endpoint to retrieve all products
@app.get("/products/")
async def get_products():
    db = SessionLocal()
    return db.query(Product).all()


# Endpoint to retrieve a specific product by ID
@app.get("/products/{product_id}")
async def get_product(product_id: int):
    db = SessionLocal()
    product = db.query(Product).filter(Product.id == product_id).first()
    if product is None:
        raise HTTPException(status_code=404, detail="Product not found")
    return product


# Endpoint to update a product by ID
@app.put("/products/{product_id}")
async def update_product(product_id: int, product: ProductCreate):
    db = SessionLocal()
    db_product = db.query(Product).filter(Product.id == product_id).first()
    if db_product is None:
        raise HTTPException(status_code=404, detail="Product not found")
    for attr, value in product.dict().items():
        setattr(db_product, attr, value)
    db.commit()
    db.refresh(db_product)
    return db_product


# Endpoint to delete a product by ID
@app.delete("/products/{product_id}")
async def delete_product(product_id: int):
    db = SessionLocal()
    db_product = db.query(Product).filter(Product.id == product_id).first()
    if db_product is None:
        raise HTTPException(status_code=404, detail="Product not found")
    db.delete(db_product)
    db.commit()
    return {"message": "Product deleted successfully"}


# Optional: Add authentication and authorization if required
