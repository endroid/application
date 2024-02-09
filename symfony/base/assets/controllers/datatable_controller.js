import { Controller } from '@hotwired/stimulus';
import DataTable from 'datatables.net';

export default class extends Controller {
    initialize() {
        let options = {
            'info': this.data.get('info') === 'true',
            'paging': this.data.get('paging') === 'true',
            'searching': this.data.get('search') === 'true',
        };
        if (this.data.get('order')) {
            let orderDirection = this.data.get('order-direction') === 'asc' ? 'asc' : 'desc';
            options['order'] = [parseInt(this.data.get('order')), orderDirection];
        }
        if (this.data.get('orderable') === 'false') {
            options['columnDefs'] = [{
                targets: "_all",
                orderable: false
            }];
        }
        new DataTable('#' + this.element.id, options);
    }
}
