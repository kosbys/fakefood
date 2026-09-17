# fakefood


# Backend flow:

User adds item to cart -> send post request -> save cart items in state -> user buys -> save purchase information in database ->
-> response can be a websocket connection that tracks the order until its finished


Endpoints
GET all items


GET item


GET cart


GET orders


POST buy order


DELETE clear past orders
