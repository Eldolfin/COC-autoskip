pipeline {
	agent any
		stages {
			stage('Checking') {
				steps {
					sh '''
					. ~/.cargo/env
					cargo clippy
					'''
				}
			}
			stage('Build') {
				steps {
					sh '''
					. ~/.cargo/env
					cargo build
					'''
				}
			}
			stage('Test') {
				steps {
					sh '''
					. ~/.cargo/env
					cargo test --release
					'''
				}
			}
		}
	post {
		unsuccessful {
			emailextrecipients([culprits()])
		}
	}
}
