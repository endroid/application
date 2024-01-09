import { Controller } from '@hotwired/stimulus';
import JsConfetti from 'js-confetti';

export default class extends Controller {
    explode() {
        const confetti = new JsConfetti();
        confetti.addConfetti();
    }
}
