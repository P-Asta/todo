class Post extends HTMLElement {

    connectedCallback() {
        let title = this.getAttribute('t');
        let discription = this.getAttribute('d');
        let ok = this.getAttributeNames('ok').indexOf("ok") != -1;

        /*
            fc: font color
            bc: background color
        */
        // this.innerHTML = `
        //     <article style="${ok? '--bc: #faffbf; --fc: #000': '--bg: #fff; --fc: #bbb'}">
        //         <h1>${title}</h1>
        //         <hr/>
        //         <section>${discription}</section>
        //     </article>
        // `

        this.innerHTML = `
            <article style="${ok? '--fc: #000': '--fc: #000'}">
                <img src="${ok?'https://raw.githubusercontent.com/5-23/todo/d8c5d94ed9656877ef38232bd320cd1eeaf5b4d0/assets/post_ok.svg': 'https://raw.githubusercontent.com/5-23/todo/d8c5d94ed9656877ef38232bd320cd1eeaf5b4d0/assets/post.svg'}"></img>
                <h1>${title}</h1>
                <section>${discription}</section>
            </article>
        `
    }

}

customElements.define("my-post", Post);