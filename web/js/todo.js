class Post extends HTMLElement {

    connectedCallback() {
        let title = this.getAttribute('t');
        let discription = this.getAttribute('d');
        let ok = this.getAttributeNames('ok').indexOf("ok") != -1;

        /*
            fc: font color
            bc: background color
        */
        this.innerHTML = `
            <article style="${ok? '--bc: #faffbf; --fc: #000': '--bg: #fff; --fc: #bbb'}">
                <h1>${title}</h1>
                <hr/>
                <section>${discription}</section>
            </article>
        `
    }

}

customElements.define("my-post", Post);