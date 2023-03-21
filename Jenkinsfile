pipeline {
	agent any
		stages {
			stage('Checking') {
				steps {
					echo 'Running clippy..'
						sh cargo clippy
				}
			}
			stage('Build') {
				steps {
					echo 'Building..'
						sh cargo build
				}
			}
			stage('Test') {
				steps {
					echo 'Testing..'
						sh cargo test --release
				}
			}
			stage('Deploy') {
				steps {
					echo 'Deploying....'
				}
			}

		}
	post {
//		success {
//			// One or more steps need to be included within each condition's block.
//		}
		unsuccessful {
			// email people who caused the build to fail
			emailextrecipients([culprits()])
		}
	}
}
