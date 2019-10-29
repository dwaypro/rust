import appHTML from "./app.html";
import appCSS from "./app.css";
const wasmJs = import ("../services/pkg/helloworld.js");

class App extends HTMLElement {

    constructor() {
        super();

        const shadowRoot = this.attachShadow({
            mode: 'open'
        });

        const template = document.createElement('template');
        template.innerHTML = `
        <style>
            ${appCSS}
        </style>
            ${appHTML}                      
        `
        shadowRoot.appendChild(template.content.cloneNode(true));
    }


    attributeChangedCallback(name, oldValue, newValue) {

    }

    connectedCallback() {
        var appInitialized = this.appInitialized;
        console.log('appInitialized', appInitialized);
        if(!appInitialized){
            this.setAppInitialized(true);
            console.log('made it here!');
            wasmJs.then(function(js){
                console.log('js', js);
                js.greet('Web Assembly');

            })
        }
    }

    setAppInitialized(boolean){
        this.appInitialized = boolean;
    }
}

customElements.define('new-app', App);

export {App}
            
        