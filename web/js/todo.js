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
        let postOK = "https://raw.githubusercontent.com/5-23/todo/9786f94188119d1c68deb5bebeb41577ac9d1a43/assets/post.svg"
        let post = "https://raw.githubusercontent.com/5-23/todo/9786f94188119d1c68deb5bebeb41577ac9d1a43/assets/post.svg"

        this.innerHTML = `
            <article style="${ok? '--fc: #fff498': '--fc: #000'}">
                <img src="${ok? postOK: post}"></img>
                <h1>${title}</h1>
                <section>${discription}</section>
            </article>
        `
    }

}

customElements.define("my-post", Post);