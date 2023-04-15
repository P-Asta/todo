tUp = 0.5
t = tUp

class Post extends HTMLElement {
    
    connectedCallback() {
        let title = this.getAttribute('t');
        let discription = this.getAttribute('d');
        let ok = this.getAttributeNames('ok').indexOf("ok") != -1;

        /*
            fc: font color
            bc: background color
        */
        let postOK = "https://raw.githubusercontent.com/5-23/todo/main/assets/post_ok.svg"
        let post = "https://raw.githubusercontent.com/5-23/todo/main/assets/post.svg"

        this.innerHTML = `
            <article style="${ok? '--fc: #000': '--fc: #000'}; --t: ${t += tUp}s">
                <img src="${ok? postOK: post}"></img>
                <h1>${title}</h1>
                <section>${discription}</section>
            </article>
        `
    }

}

customElements.define("my-post", Post);