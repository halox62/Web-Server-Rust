from flask import Flask, jsonify, send_from_directory



app = Flask(__name__, static_folder='../frontend')

@app.route('/')
def index():
    return send_from_directory(app.static_folder, 'index.html')

@app.route('/api/data')
def api_data():
    return jsonify({
        "message": "Ciao dal backend Flask!",
        "status": "success"
    })

if __name__ == "__main__":
    app.run(port=5000)