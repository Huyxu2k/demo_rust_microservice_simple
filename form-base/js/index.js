(
    function(){
        let book=null;

        //CONST
        const appLoadingDisplay = document.getElementById("app-loading-display");
        const bookDisplay = document.getElementById("book-display");
        const bookTable = document.getElementById("book-table");
        const bookEmptyText = document.getElementById("book-empty-text");
        const bookTableTbody = document.querySelector("#book-table tbody");
        const addBookWrapper= document.getElementById("add-book-wrapper");
        const addBookForm = document.getElementById("add-book-form");
        //const ID = document.getElementById("ID");
        const bookNameField = document.getElementById("book-name");
        const authorField = document.getElementById("author");
        const amountField = document.getElementById("amount");
        const cagetoryField = document.getElementById("cagetory");
        const noteField = document.getElementById("note");
        
        //Load List Book From 
        function fetchBooks() {
            fetch("http://localhost:8080/books")
              .then(r => r.json())
              .then(r => books = r)
              .then(renderBooks)
              .catch((e) => {
                init();
              });
        }

        //Render Row In Table
        function renderBooks() {
            appLoadingDisplay.classList.add("d-none");
            bookDisplay.classList.remove("d-none");
            addBookWrapper.classList.remove("d-none");
        
            if (books.length === 0) {
              bookEmptyText.classList.remove("d-none");
              bookTable.classList.add("d-none");
              return;
            }
            bookEmptyText.classList.add("d-none");
            bookTable.classList.remove("d-none");
        
            while (bookTableTbody.firstChild) {
                bookTableTbody.removeChild(bookTableTbody.firstChild);
            }
        
            books.forEach((book) => {
              //const ID = book.ID;
              const row = document.createElement("tr");
              row.appendChild(createCell(book.id));
              row.appendChild(createCell(book.book_name));
              row.appendChild(createCell(book.author));
              row.appendChild(createCell(book.amount));
              row.appendChild(createCell(book.category));
              row.appendChild(createCell(book.note));
              const actionCell = document.createElement("td");
        
              const deleteButton = document.createElement("button");
              deleteButton.classList.add(...["btn","btn-sm","btn-danger"]);
              deleteButton.innerText = "Delete";
        
              deleteButton.addEventListener("click", (e) => {
                e.preventDefault();
                deleteBook(id);
              });
        
              actionCell.appendChild(deleteButton);
              row.appendChild(actionCell);
              bookTableTbody.appendChild(row);
            });
          }
          
          //Delete Book By ID
          function deleteBook(id) {
            fetch(`http://localhost:8080/delete_book?id=${id}`)
              .then(() => fetchBooks());
          }

          //Create Cell
          function createCell(contents) {
            const cell = document.createElement("td");
            cell.innerText = contents;
            return cell;
          }

          //Show Error
          function displayError(err) {
            alert("Error:" + err);
          }
        
          //ADD Book After On Submit
          function onAddFormSubmit(e) {
            e.preventDefault();
        
            const data = {
              book_name : bookNameField.value,
              author : authorField.value,
              amount : parseInt(amountField.value),
              category : cagetoryField.value,
              note : noteField.value,
            };
        
            fetch("http://localhost:8080/create_book", {
              method: "POST",
              body: JSON.stringify(data),
              headers: { "Content-type": "application/json" },
            }).then(() => fetchBooks())
              .then(() => resetAddBookForm());
        
            alert("Book added");
          }
        
          // Reset Form
          function resetAddBookForm() {
            bookEmptyText.value = "";
            authorField.value = "";
            amountField.value = "";
            cagetoryField.value = "";
            noteField.value = "";
          }
          fetchBooks();
          addBookForm.addEventListener("submit", onAddFormSubmit);
    }
)();